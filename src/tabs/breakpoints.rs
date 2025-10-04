use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct BreakpointsTab;

impl BreakpointsTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for BreakpointsTab {
    fn title(&self) -> &'static str {
        "Breakpoints"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.heading("Breakpoints");

            let mut to_remove = Vec::new();
            for (i, bp) in app.breakpoints.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", i + 1, bp));
                    if ui.button("Remove").clicked() {
                        to_remove.push(i);
                    }
                });
            }

            // Remove items in reverse order to maintain indices
            for &i in to_remove.iter().rev() {
                app.breakpoints.remove(i);
            }

            if ui.button("Add Breakpoint").clicked() {
                // TODO: Implement breakpoint addition
            }
        });
    }
}