use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct StackTab;

impl StackTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for StackTab {
    fn title(&self) -> &'static str {
        "Stack"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Stack"));
    }
}