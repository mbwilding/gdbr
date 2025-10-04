use egui::{ScrollArea, TextEdit, TextStyle, Ui, WidgetText};
use egui_dock::TabViewer;
use serde::{Deserialize, Serialize};

// Tab types for the dock interface
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Tab {
    Content,
    Console,
    Exe,
    Breakpoints,
    Commands,
    Struct,
    Stack,
    Files,
    Thread,
    CmdSearch,
    Watch,
    Locals,
    Registers,
    Data,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Self::Content => "Content",
            Self::Console => "Console",
            Self::Exe => "Exe",
            Self::Breakpoints => "Breakpoints",
            Self::Commands => "Commands",
            Self::Struct => "Struct",
            Self::Stack => "Stack",
            Self::Files => "Files",
            Self::Thread => "Thread",
            Self::CmdSearch => "CmdSearch",
            Self::Watch => "Watch",
            Self::Locals => "Locals",
            Self::Registers => "Registers",
            Self::Data => "Data",
        }
    }
}

#[derive(Debug, Default)]
pub struct Tabs {
    console_input: String,
    logs: String,
}

impl TabViewer for Tabs {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Content => {
                ui.centered_and_justified(|ui| ui.heading("Content"));
            }
            Tab::Console => {
                ui.vertical(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), ui.available_height() - 30.0), // Reserve space for input
                        egui::Layout::top_down(egui::Align::default()),
                        |ui| {
                            ScrollArea::new([true, true])
                                .auto_shrink(false)
                                .show(ui, |ui| {
                                    ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(&mut self.logs)
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
                            TextEdit::singleline(&mut self.console_input)
                                .font(TextStyle::Monospace),
                        );

                        if response.lost_focus() {
                            response.request_focus();
                            // TODO: Handle command entered
                            self.logs.push_str(&self.console_input);
                            self.logs.push('\n');
                            self.console_input.clear();
                        };
                    });
                });
            }
            Tab::Exe => {
                ui.centered_and_justified(|ui| ui.heading("Exe"));
            }
            Tab::Breakpoints => {
                ui.centered_and_justified(|ui| ui.heading("Breakpoints"));
            }
            Tab::Commands => {
                ui.centered_and_justified(|ui| ui.heading("Commands"));
            }
            Tab::Struct => {
                ui.centered_and_justified(|ui| ui.heading("Struct"));
            }
            Tab::Stack => {
                ui.centered_and_justified(|ui| ui.heading("Stack"));
            }
            Tab::Files => {
                ui.centered_and_justified(|ui| ui.heading("Files"));
            }
            Tab::Thread => {
                ui.centered_and_justified(|ui| ui.heading("Thread"));
            }
            Tab::CmdSearch => {
                ui.centered_and_justified(|ui| ui.heading("CmdSearch"));
            }
            Tab::Watch => {
                ui.centered_and_justified(|ui| ui.heading("Watch"));
            }
            Tab::Locals => {
                ui.centered_and_justified(|ui| ui.heading("Locals"));
            }
            Tab::Registers => {
                ui.centered_and_justified(|ui| ui.heading("Registers"));
            }
            Tab::Data => {
                ui.centered_and_justified(|ui| ui.heading("Data"));
            }
        }
    }
}
