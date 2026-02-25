//! Launch orchestration: version resolution, modloader setup, and process spawn.

use crate::options::LaunchOptions;
use crate::runtime;
use crate::version;
use std::fmt;

/// Handle to a launched game process (for future use: wait, kill, etc.).
#[derive(Debug)]
pub struct LaunchHandle {
    _private: (),
}

/// Errors that can occur during launch.
#[derive(Debug)]
pub struct LaunchError {
    message: String,
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LaunchError {}

impl LaunchError {
    fn new(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

/// Launches Minecraft with the given options.
///
/// Resolves game version, downloads client and libraries if needed, then spawns the game process.
/// Blocks until the game process exits. Only Vanilla is supported currently; Fabric/Forge/Quilt
/// will be supported in a future release.
pub fn launch(options: &LaunchOptions) -> Result<LaunchHandle, LaunchError> {
    // Only Vanilla supported for now
    if !matches!(options.modloader, crate::options::ModLoader::Vanilla) {
        return Err(LaunchError::new(
            "Only Vanilla Minecraft is supported currently. Use ModLoader::Vanilla. \
             Fabric, Forge, and Quilt support coming soon.",
        ));
    }

    let manifest = version::fetch_manifest().map_err(LaunchError::new)?;
    let version_url =
        version::find_version_url(&manifest, &options.game_version).ok_or_else(|| {
            LaunchError::new(format!(
                "Game version '{}' not found in manifest",
                options.game_version
            ))
        })?;

    let version_json = version::fetch_version_json(&version_url).map_err(LaunchError::new)?;

    let game_dir = options
        .instance_dir
        .clone()
        .unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "mc-launcher", "mc-launcher")
                .expect("project dirs")
                .data_dir()
                .join("game")
        });

    let java_path = options.java_path.as_deref();

    let mut child = runtime::run(
        &version_json,
        &game_dir,
        java_path,
        options.memory_mb,
    ).map_err(LaunchError::new)?;

    let _ = child.wait();

    Ok(LaunchHandle { _private: () })
}
