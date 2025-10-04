use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct CmdSearchTab;

impl CmdSearchTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for CmdSearchTab {
    fn title(&self) -> &'static str {
        "CmdSearch"
    }

    fn ui(&mut self, ui: &mut Ui, _app: &mut Gdbr) {
        ui.centered_and_justified(|ui| ui.heading("CmdSearch"));
    }
}