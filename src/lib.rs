pub mod error;
pub mod keybind_switcher;
pub mod json_usage;
pub mod data;
pub mod gui;

/// Trait for things that can generate source command
pub trait GenerateCommand {
    fn generate(&self) -> error::Result<String>;
}
