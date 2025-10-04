use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct DataTab;

impl DataTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for DataTab {
    fn title(&self) -> &'static str {
        "Data"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Data"));
    }
}