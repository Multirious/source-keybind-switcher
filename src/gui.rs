use eframe::egui;
use crate::{json_usage::JsonUsage, keybind_switcher::{KeybindSwitcher, CommandSet}};

pub fn launch() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Source Keybind Switcher", native_options, Box::new(|cc| Box::new(GUI::new(cc))));
}

#[derive(Debug, Default)]
pub struct GUI {
    json_usage: JsonUsage,
    switcher: KeybindSwitcher,
    set_index_editing: Option<usize>,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Switcher");
            ui.text_edit_singleline(&mut self.switcher.name);

            ui.label("Set Name");
            match self.set_index_editing {
                Some(i) => ui.add_enabled_ui(true, |ui| {
                    ui.text_edit_singleline(&mut self.switcher.command_sets[i].name);
                }),
                None => ui.add_enabled_ui(false, |ui| {
                    ui.text_edit_singleline(&mut "No command set selected");
                }),
            };

            // ui.text_edit_singleline(&mut self.switcher.);
        });
   }
}