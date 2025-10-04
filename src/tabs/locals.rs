use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct LocalsTab;

impl LocalsTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for LocalsTab {
    fn title(&self) -> &'static str {
        "Locals"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.heading("Local Variables");

            for (i, local) in app.locals.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", i + 1, local));
                });
            }
        });
    }
}