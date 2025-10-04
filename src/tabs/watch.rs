use super::TabImpl;
use crate::app::Gdbr;
use egui::Ui;

pub struct WatchTab;

impl WatchTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for WatchTab {
    fn title(&self) -> &'static str {
        "Watch"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.heading("Watch Variables");

            let mut to_remove = Vec::new();
            for (i, var) in app.watch_vars.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", i + 1, var));
                    if ui.button("Remove").clicked() {
                        to_remove.push(i);
                    }
                });
            }

            // Remove items in reverse order to maintain indices
            for &i in to_remove.iter().rev() {
                app.watch_vars.remove(i);
            }

            if ui.button("Add Watch Variable").clicked() {
                // TODO: Implement watch variable addition
            }
        });
    }
}