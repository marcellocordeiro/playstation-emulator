use std::sync::{Arc, Mutex};

use egui::Window;
use playstation_core::PlayStation;

use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct Control {
    opened: bool,
    running: Arc<Mutex<bool>>,
}

impl Control {
    pub fn new(running: Arc<Mutex<bool>>) -> Self {
        Self {
            opened: false,
            running,
        }
    }

    pub fn draw_manual_control_button(ctx: &Gui, ui: &mut egui::Ui) {
        let mut running = ctx.control.running.lock().unwrap();

        let text = if *running { "Auto" } else { "Manual" };

        if ui.button(text).clicked() {
            *running = !*running;
        }
    }

    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut egui::Ui) {
        if ui.button("Control").clicked() {
            ctx.control.opened = !ctx.control.opened;
        }
    }

    pub fn draw(ctx: &mut Gui, ui: &egui::Ui, mut ps: Option<&mut PlayStation>) {
        if !ctx.control.opened {
            return;
        }

        Window::new("Control")
            .open(&mut ctx.control.opened)
            .show(ui, |ui| {
                let enable_buttons = *ctx.control.running.lock().unwrap();

                ui.add_enabled_ui(enable_buttons, |ui| {
                    if ui.button("Step").clicked() {
                        if let Some(ps) = ps.as_deref_mut() {
                            ps.step();
                        }
                    }

                    if ui.button("Run frame").clicked() {
                        if let Some(ps) = ps {
                            ps.run_frame();
                        }
                    }
                });
            });
    }
}
