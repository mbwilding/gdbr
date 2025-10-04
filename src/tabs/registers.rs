use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct RegistersTab;

impl RegistersTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for RegistersTab {
    fn title(&self) -> &'static str {
        "Registers"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.heading("Registers");

            for (i, reg) in app.registers.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", i + 1, reg));
                });
            }
        });
    }
}