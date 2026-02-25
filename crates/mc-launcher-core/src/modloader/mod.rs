//! Modloader-specific logic.
//!
//! Given game version + modloader type/version, produces what is needed
//! for that combination (e.g. Fabric installer output, Forge installer, Quilt).

mod fabric;
mod forge;
mod quilt;
mod vanilla;
