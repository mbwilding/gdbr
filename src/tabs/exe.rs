use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct ExeTab;

impl ExeTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for ExeTab {
    fn title(&self) -> &'static str {
        "Exe"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Exe"));
    }
}