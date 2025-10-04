use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct ContentTab;

impl ContentTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for ContentTab {
    fn title(&self) -> &'static str {
        "Content"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Content"));
    }
}