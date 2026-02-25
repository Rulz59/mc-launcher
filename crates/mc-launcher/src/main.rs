//! Entry point: build launch options and call into core.
//!
//! No game-running logic here; core handles version resolution, modloader, and process spawn.

use mc_launcher_core::{launch, LaunchOptions, ModLoader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = LaunchOptions {
        // Use 1.20.1 for Java 17 compatibility; 1.21.x requires Java 21.
        game_version: "1.20.1".to_string(),
        modloader: ModLoader::Vanilla,
        instance_dir: None,
        java_path: None,
        memory_mb: Some(2048),
    };

    let _handle = launch(&options)?;
    Ok(())
}
