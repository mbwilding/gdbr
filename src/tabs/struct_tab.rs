use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct StructTab;

impl StructTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for StructTab {
    fn title(&self) -> &'static str {
        "Struct"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Struct"));
    }
}