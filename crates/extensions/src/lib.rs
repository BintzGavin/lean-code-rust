// Minimal lean-extensions crate initialization
pub mod manifest;
pub use manifest::ExtensionManifest;
pub mod vsix;

pub mod host;
pub use host::ExtensionHost;
