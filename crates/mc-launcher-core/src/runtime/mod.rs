//! Runtime: download client/libraries and spawn the game process.

use crate::version::VersionJson;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn current_os_name() -> &'static str {
    if std::cfg!(target_os = "windows") {
        "windows"
    } else if std::cfg!(target_os = "macos") {
        "osx"
    } else {
        "linux"
    }
}

fn library_allowed_by_rules(rules: &Option<Vec<crate::version::Rule>>) -> bool {
    let rules = match rules {
        Some(r) => r,
        None => return true,
    };
    for rule in rules {
        if let Some(ref os) = rule.os {
            if let Some(ref name) = os.name {
                if name != current_os_name() {
                    return false;
                }
            }
        }
        if rule.action.as_deref() == Some("disallow") {
            return false;
        }
    }
    true
}

fn jvm_arg_allowed(entry: &crate::version::ArgumentEntry) -> bool {
    match entry {
        crate::version::ArgumentEntry::String(_) => true,
        crate::version::ArgumentEntry::Object { rules, .. } => {
            let rules = match rules {
                Some(r) => r,
                None => return true,
            };
            for rule in rules {
                if let Some(ref os) = rule.os {
                    if let Some(ref name) = os.name {
                        if name != current_os_name() {
                            return false;
                        }
                    }
                }
                if rule.action.as_deref() == Some("disallow") {
                    return false;
                }
            }
            true
        }
    }
}

fn expand_jvm_args(
    args: &[crate::version::ArgumentEntry],
    classpath: &str,
    natives_dir: &Path,
    classpath_sep: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    for entry in args {
        if !jvm_arg_allowed(entry) {
            continue;
        }
        match entry {
            crate::version::ArgumentEntry::String(s) => {
                let s = s
                    .replace("${classpath_separator}", classpath_sep)
                    .replace("${classpath}", classpath)
                    .replace(
                        "${natives_directory}",
                        natives_dir.to_string_lossy().as_ref(),
                    )
                    .replace("${launcher_name}", "mc-launcher")
                    .replace("${launcher_version}", "0.1.0");
                out.push(s);
            }
            crate::version::ArgumentEntry::Object { value, .. } => {
                for s in value {
                    let s = s
                        .replace("${classpath_separator}", classpath_sep)
                        .replace("${classpath}", classpath)
                        .replace(
                            "${natives_directory}",
                            natives_dir.to_string_lossy().as_ref(),
                        )
                        .replace("${launcher_name}", "mc-launcher")
                        .replace("${launcher_version}", "0.1.0");
                    out.push(s);
                }
            }
        }
    }
    out
}

fn download_file(url: &str, path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    let parent = path.parent().ok_or("invalid path")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let resp = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("download failed {}: {}", resp.status(), url));
    }
    let bytes = resp.bytes().map_err(|e| e.to_string())?;
    fs::write(path, &bytes).map_err(|e| e.to_string())?;
    Ok(())
}

/// Prepare game files (client + libraries) and run the game.
pub fn run(
    version: &VersionJson,
    game_dir: &Path,
    java_path: Option<&Path>,
    memory_mb: Option<u32>,
) -> Result<std::process::Child, String> {
    let versions_dir = game_dir.join("versions").join(&version.id);
    let libs_dir = game_dir.join("libraries");
    let client_jar = versions_dir.join(format!("{}.jar", version.id));
    let natives_dir = versions_dir.join("natives");

    fs::create_dir_all(&versions_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&libs_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&natives_dir).map_err(|e| e.to_string())?;

    // Download client
    download_file(&version.downloads.client.url, &client_jar)?;

    let classpath_sep = if std::cfg!(target_os = "windows") {
        ";"
    } else {
        ":"
    };

    let mut classpath_parts = vec![client_jar];

    for lib in &version.libraries {
        if !library_allowed_by_rules(&lib.rules) {
            continue;
        }
        if lib.natives.is_some() {
            // Skip native libs for classpath (they get extracted to natives dir)
            continue;
        }
        let artifact = match &lib.downloads {
            Some(d) => match &d.artifact {
                Some(a) => a,
                None => continue,
            },
            None => continue,
        };
        let path = libs_dir.join(&artifact.path);
        download_file(&artifact.url, &path)?;
        classpath_parts.push(path);
    }

    let classpath = classpath_parts
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(classpath_sep);

    let mut jvm_args: Vec<String> = Vec::new();

    if let Some(mb) = memory_mb {
        jvm_args.push(format!("-Xmx{}M", mb));
        jvm_args.push(format!("-Xms{}M", mb.min(256)));
    }

    if let Some(ref args) = version.arguments {
        if let Some(ref jvm) = args.jvm {
            jvm_args.extend(expand_jvm_args(
                jvm,
                &classpath,
                &natives_dir,
                classpath_sep,
            ));
        }
    }

    // Fallback if no jvm args in version json (old versions)
    if jvm_args.is_empty() || !jvm_args.iter().any(|a| a.contains("-cp") || a == "-cp") {
        jvm_args.push(format!("-Djava.library.path={}", natives_dir.display()));
        jvm_args.push("-cp".to_string());
        jvm_args.push(classpath);
    }

    let game_args: Vec<String> = if let Some(ref args) = version.arguments {
        if let Some(ref game) = args.game {
            let mut out = Vec::new();
            for entry in game {
                if !jvm_arg_allowed(entry) {
                    continue;
                }
                match entry {
                    crate::version::ArgumentEntry::String(s) => {
                        out.push(replace_game_vars(s, game_dir));
                    }
                    crate::version::ArgumentEntry::Object { value, .. } => {
                        for s in value {
                            out.push(replace_game_vars(s, game_dir));
                        }
                    }
                }
            }
            out
        } else {
            default_game_args(game_dir)
        }
    } else if let Some(ref legacy) = version.minecraft_arguments_legacy {
        legacy
            .split_whitespace()
            .map(|s| replace_game_vars(s, game_dir))
            .collect()
    } else {
        default_game_args(game_dir)
    };

    let java = find_java(java_path)?;

    let mut cmd = Command::new(&java);
    cmd.args(&jvm_args).arg(&version.main_class).args(&game_args);
    cmd.current_dir(game_dir);

    cmd.spawn().map_err(|e| e.to_string())
}

fn replace_game_vars(s: &str, game_dir: &Path) -> String {
    s.replace("${game_directory}", game_dir.to_string_lossy().as_ref())
        .replace("${version_name}", "mc-launcher")
        .replace("${assets_index}", "default")
        .replace("${assets_root}", "assets")
        .replace("${auth_uuid}", "00000000-0000-0000-0000-000000000000")
        .replace("${auth_access_token}", "0")
        .replace("${auth_session}", "0")
        .replace("${user_type}", "legacy")
        .replace("${username}", "Player")
}

fn default_game_args(game_dir: &Path) -> Vec<String> {
    vec![
        "--gameDir".to_string(),
        game_dir.to_string_lossy().into_owned(),
        "--username".to_string(),
        "Player".to_string(),
    ]
}

fn find_java(custom: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(p) = custom {
        if p.is_file() {
            return Ok(p.to_path_buf());
        }
        let java = p.join("bin").join(java_bin_name());
        if java.is_file() {
            return Ok(java);
        }
        return Err(format!("Java not found at {}", p.display()));
    }
    if let Ok(home) = std::env::var("JAVA_HOME") {
        let path = PathBuf::from(home);
        let java = path.join("bin").join(java_bin_name());
        if java.is_file() {
            return Ok(java);
        }
    }
    // Assume "java" is on PATH
    Ok(PathBuf::from("java"))
}

fn java_bin_name() -> &'static str {
    if std::cfg!(target_os = "windows") {
        "java.exe"
    } else {
        "java"
    }
}
