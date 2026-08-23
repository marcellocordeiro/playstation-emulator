use std::sync::{Arc, Mutex};

use eframe::Storage;
use egui::ViewportCommand;
use playstation_core::PlayStation;

use crate::{
    file_manager::FileManager,
    gui::{Event, Gui},
};

pub struct App {
    ps: Option<PlayStation>,
    file_manager: FileManager,
    gui: Gui,

    running: Arc<Mutex<bool>>,
}

impl App {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext, file_manager: FileManager) -> Self {
        //let ps = PlayStation::new(fil)
        let ps = {
            if let Some(bios) = &file_manager.bios {
                let bios_data = bios.data.clone();

                Some(PlayStation::new(*bios_data))
            } else {
                None
            }
        };

        let running = Arc::from(Mutex::<bool>::default());

        Self {
            ps,
            file_manager,
            gui: Gui::new(&cc.egui_ctx, running.clone()),
            running,
        }
    }

    fn handle_events(&self, _storage: Option<&dyn Storage>, ui: &egui::Context) {
        ui.input(|i| {
            use egui::Key;

            if i.key_pressed(Key::Escape) {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
        });

        while let Ok(event) = self.gui.event_receiver.try_recv() {
            match event {
                Event::BiosSelected(_file) => {
                    // self.file_manager.bootrom = Some(file);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, eframe_frame: &mut eframe::Frame) {
        self.handle_events(eframe_frame.storage(), ui);

        if *self.running.lock().unwrap()
            && let Some(ps) = &mut self.ps
        {
            ps.step();
        }

        self.gui.render(ui, self.ps.as_mut());

        ui.request_repaint();
    }
}
