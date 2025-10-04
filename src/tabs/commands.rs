use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct CommandsTab;

impl CommandsTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for CommandsTab {
    fn title(&self) -> &'static str {
        "Commands"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Commands"));
    }
}