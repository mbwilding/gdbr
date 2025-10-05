use crate::gdb::Gdb;
use egui::{RichText, ScrollArea, TextEdit, TextStyle, Ui, WidgetText};
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
    console_input_prev: String,
    logs: Vec<String>,
    gdb_available: bool,
    pending_commands: Vec<String>,
}

impl Tabs {
    /// Update logs with GDB output
    pub fn update_from_gdb(&mut self, gdb: &Gdb) {
        while let Some(output) = gdb.try_receive_output() {
            self.logs.push(output);
        }
    }

    /// Send a command to GDB and add it to logs
    pub fn send_command_to_gdb(
        &mut self,
        command: &str,
        gdb: &Gdb,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.logs.push(format!("> {command}"));

        gdb.send_command(command.to_owned())?;

        Ok(())
    }

    /// Set GDB availability
    pub fn set_gdb_available(&mut self, available: bool) {
        self.gdb_available = available;
    }

    /// Get GDB availability
    pub fn is_gdb_available(&self) -> bool {
        self.gdb_available
    }

    /// Get pending commands and clear the queue
    pub fn take_pending_commands(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_commands)
    }

    /// Add a command to the pending queue
    pub fn add_pending_command(&mut self, command: String) {
        self.pending_commands.push(command);
    }
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
                        egui::Vec2::new(ui.available_width(), ui.available_height() - 30.0),
                        egui::Layout::top_down(egui::Align::default()),
                        |ui| {
                            ScrollArea::new([true, true])
                                .auto_shrink(false)
                                .show(ui, |ui| {
                                    for log_entry in &self.logs {
                                        ui.label(RichText::new(log_entry).monospace());
                                    }
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

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            response.request_focus();
                            if self.console_input.is_empty() {
                                // Repeat last command
                                if !self.console_input_prev.is_empty() {
                                    self.add_pending_command(self.console_input_prev.clone());
                                }
                            } else {
                                // Add command to pending queue for GDB processing
                                self.add_pending_command(self.console_input.clone());
                                if self.console_input_prev != self.console_input {
                                    self.console_input_prev = self.console_input.clone();
                                }
                                self.console_input.clear();
                            }
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
