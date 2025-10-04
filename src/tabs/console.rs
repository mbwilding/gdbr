use super::TabImpl;
use crate::app::Gdbr;
use egui::{ScrollArea, TextEdit, TextStyle, Ui};

pub struct ConsoleTab;

impl ConsoleTab {
    pub fn new() -> Self {
        Self
    }
}

impl TabImpl for ConsoleTab {
    fn title(&self) -> &'static str {
        "Console"
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut Gdbr) {
        ui.vertical(|ui| {
            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), ui.available_height() - 30.0),
                egui::Layout::top_down(egui::Align::default()),
                |ui| {
                    ScrollArea::new([true, true])
                        .auto_shrink(false)
                        .show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                TextEdit::multiline(&mut app.logs)
                                    .font(TextStyle::Monospace)
                                    .interactive(false)
                                    .frame(false),
                            );
                        });
                },
            );

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Command");
                let response = ui.add_sized(
                    ui.available_size(),
                    TextEdit::singleline(&mut app.console_input)
                        .font(TextStyle::Monospace),
                );

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    response.request_focus();
                    if app.console_input.is_empty() {
                        app.logs.push_str(&app.console_input_prev);
                        app.logs.push('\n');
                    } else {
                        app.logs.push_str(&app.console_input);
                        app.logs.push('\n');
                        if app.console_input_prev != app.console_input {
                            app.console_input_prev = app.console_input.clone();
                        }
                        app.console_input.clear();
                    }
                };
            });
        });
    }
}