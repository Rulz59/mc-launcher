//! Game version resolution (manifest, version json).
//!
//! Fetches/resolves game version from Mojang Piston Meta API.

use serde::Deserialize;

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Deserialize)]
pub struct VersionManifestV2 {
    pub versions: Vec<VersionRef>,
}

#[derive(Debug, Deserialize)]
pub struct VersionRef {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionJson {
    pub id: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub arguments: Option<Arguments>,
    #[serde(rename = "minecraftArguments")]
    pub minecraft_arguments_legacy: Option<String>,
    pub libraries: Vec<Library>,
    pub downloads: Downloads,
}

#[derive(Debug, Deserialize)]
pub struct Arguments {
    pub jvm: Option<Vec<ArgumentEntry>>,
    pub game: Option<Vec<ArgumentEntry>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgumentEntry {
    String(String),
    Object {
        #[serde(deserialize_with = "deserialize_value_vec")]
        value: Vec<String>,
        rules: Option<Vec<Rule>>,
    },
}

fn deserialize_value_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let v: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    match v {
        serde_json::Value::String(s) => Ok(vec![s]),
        serde_json::Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for x in arr {
                match x {
                    serde_json::Value::String(s) => out.push(s),
                    _ => return Err(D::Error::custom("expected string in argument value array")),
                }
            }
            Ok(out)
        }
        _ => Err(D::Error::custom("argument value must be string or array of strings")),
    }
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: Option<String>,
    pub os: Option<OsRule>,
}

#[derive(Debug, Deserialize)]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub client: ClientDownload,
}

#[derive(Debug, Deserialize)]
pub struct ClientDownload {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<Rule>>,
    pub natives: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
}

#[derive(Debug, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

/// Fetch version manifest from Mojang.
pub fn fetch_manifest() -> Result<VersionManifestV2, String> {
    let resp = reqwest::blocking::get(MANIFEST_URL).map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("manifest request failed {}: {}", status, body));
    }
    serde_json::from_str(&body).map_err(|e| e.to_string())
}

/// Find version URL by game version id.
pub fn find_version_url(manifest: &VersionManifestV2, version_id: &str) -> Option<String> {
    manifest
        .versions
        .iter()
        .find(|v| v.id == version_id)
        .map(|v| v.url.clone())
}

/// Fetch version json from URL.
pub fn fetch_version_json(url: &str) -> Result<VersionJson, String> {
    let resp = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
    let status = resp.status();
    let body = resp.text().map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("version request failed {}: {}", status, body));
    }
    serde_json::from_str(&body).map_err(|e| e.to_string())
}
