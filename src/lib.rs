pub mod error;
pub mod keybind_switcher;
pub mod program;
pub mod data;
pub mod ui;

/// Trait for things that can generate source command
pub trait GenerateCommand {
    fn generate(&self) -> error::Result<String>;
}
