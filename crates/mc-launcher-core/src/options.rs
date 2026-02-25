//! Launch options and modloader types for mc-launcher.

use std::path::PathBuf;

/// Modloader type and optional loader version.
#[derive(Debug, Clone)]
pub enum ModLoader {
    /// Vanilla Minecraft (no modloader).
    Vanilla,
    /// Fabric with the given loader version.
    Fabric(String),
    /// Forge with the given installer version.
    Forge(String),
    /// Quilt with the given loader version.
    Quilt(String),
}

/// Options for launching a Minecraft instance.
#[derive(Debug, Clone)]
pub struct LaunchOptions {
    /// Game version (e.g. "1.20.1").
    pub game_version: String,
    /// Modloader and its version.
    pub modloader: ModLoader,
    /// Optional instance directory (game dir, mods, config, etc.).
    pub instance_dir: Option<PathBuf>,
    /// Optional path to the Java executable.
    pub java_path: Option<PathBuf>,
    /// Optional memory limit in megabytes.
    pub memory_mb: Option<u32>,
}
