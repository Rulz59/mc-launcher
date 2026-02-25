//! Core library for mc-launcher.
//!
//! All game-running logic lives here: version resolution, modloader setup,
//! and process spawn. The binary calls [`launch`] with [`LaunchOptions`].

mod launch;
mod modloader;
mod options;
mod runtime;
mod version;

// Public API
pub use launch::{launch, LaunchError, LaunchHandle};
pub use options::{LaunchOptions, ModLoader};
