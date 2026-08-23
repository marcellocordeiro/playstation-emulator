use std::sync::{
    Arc,
    Mutex,
    mpsc::{Receiver, Sender},
};

use egui::{MenuBar, Panel, ViewportCommand};
use playstation_core::PlayStation;

use self::control::Control;
use crate::file_manager::{FileInfo, FileType, file_picker_async};

pub enum Event {
    BiosSelected(FileInfo),
}

pub struct Gui {
    pub event_receiver: Receiver<Event>,
    pub event_sender: Sender<Event>,

    pub control: Control,
    //pub error: Option<Box<dyn Error>>,
    //pub message: Option<String>,
}

impl Gui {
    pub fn new(_egui_ctx: &egui::Context, running: Arc<Mutex<bool>>) -> Self {
        let (event_sender, event_receiver) = std::sync::mpsc::channel();

        Self {
            event_receiver,
            event_sender,
            control: Control::new(running),
            //error: None,
            //message: None,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, ps: Option<&mut PlayStation>) {
        self.render_ui(ui, ps);
    }

    fn render_ui(&mut self, ui: &mut egui::Ui, mut ps: Option<&mut PlayStation>) {
        Panel::top("top_panel").show(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load BIOS").clicked() {
                        file_picker_async(FileType::Bios, self.event_sender.clone());
                    }

                    if ui.button("Reset").clicked() {
                        if let Some(ps) = ps.as_deref_mut() {
                            ps.reset();
                        }
                    }

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                Control::draw_manual_control_button(self, ui);
                Control::draw_widget_toggle_button(self, ui);
            });
        });

        Control::draw(self, ui, ps);
    }
}

mod control;
