pub mod error;
pub mod keybind_switcher;
pub mod program;
pub mod data;
mod ui;

/// Struct that can generate source command
pub trait GenerateCommand {
    fn generate(&self) -> error::Result<String>;
}
