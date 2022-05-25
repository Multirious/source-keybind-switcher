use eframe::egui;
use crate::json_usage::JsonUsage;

pub fn launch() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Source Keybind Switcher", native_options, Box::new(|cc| Box::new(GUI::new(cc))));
}

#[derive(Debug, Default)]
pub struct GUI {
    json_usage: JsonUsage,
}

impl GUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for GUI {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     egui::Frame::none()
        //         .rounding(egui::Rounding::from(5.0))
        //         .stroke(egui::Stroke::new(2.0, egui::Color32::BLACK))
        //         .inner_margin(egui::style::Margin::same(250.0))
        //         .show(ui, |ui| {
        //             panel
        //         });
        // });
   }
}