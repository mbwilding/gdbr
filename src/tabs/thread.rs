use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct ThreadTab;

impl ThreadTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for ThreadTab {
    fn title(&self) -> &'static str {
        "Thread"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("Thread"));
    }
}