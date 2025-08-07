pub mod generator;
pub mod provider;
pub mod transform;

include!(concat!(env!("OUT_DIR"), "/data.rs"));

pub use generator::Generator;
pub use provider::Source;
pub use transform::{Mode, DualMode, TransformMode};

/// Generate a random passphrase using the default settings
pub fn generate() -> String {
    Generator::new().generate()
}

/// Generate a random Chinese passphrase
pub fn generate_chinese() -> String {
    Generator::new().chinese().generate()
}
