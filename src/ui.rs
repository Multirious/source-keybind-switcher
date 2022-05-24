use eframe::egui;

use crate::program::JsonUsage;

pub fn launch() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Source Keybind Switcher", native_options, Box::new(|cc| Box::new(Program::new(cc))));
}

#[derive(Debug, Default)]
pub struct Program {
    json_usage: JsonUsage,
}

impl Program {
}

impl Program {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for Program {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}