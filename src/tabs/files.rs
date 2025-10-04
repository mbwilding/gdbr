use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct FilesTab;

impl FilesTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for FilesTab {
    fn title(&self) -> &'static str {
        "Files"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.heading("Files");

            if let Some(file) = &app.current_file {
                ui.label(format!("Current file: {}", file));
            } else {
                ui.label("No file loaded");
            }
        });
    }
}