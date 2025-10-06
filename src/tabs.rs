use crate::gdb::Gdb;
use egui::{Color32, Key, RichText, ScrollArea, TextEdit, TextStyle, Ui, WidgetText};
use egui_dock::TabViewer;
use egui_extras::syntax_highlighting::{CodeTheme, highlight};
use serde::{Deserialize, Serialize};

/// Different types of log entries with associated colors
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum LogType {
    /// User command
    Command,
    /// GDB output
    Output,
    /// Error
    Error,
    /// Info
    Info,
    /// Warning
    Warning,
}

impl LogType {
    pub fn color(&self, ui: &egui::Ui) -> Color32 {
        let is_dark = ui.visuals().dark_mode;
        match self {
            Self::Command => Color32::from_rgb(100, 150, 255),
            Self::Output => {
                if is_dark {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                }
            }
            Self::Error => {
                if is_dark {
                    Color32::from_rgb(255, 100, 100)
                } else {
                    Color32::from_rgb(200, 50, 50)
                }
            }
            Self::Info => {
                if is_dark {
                    Color32::from_rgb(100, 255, 100)
                } else {
                    Color32::from_rgb(50, 150, 50)
                }
            }
            Self::Warning => {
                if is_dark {
                    Color32::from_rgb(255, 255, 100)
                } else {
                    Color32::from_rgb(200, 150, 0)
                }
            }
        }
    }
}

/// A log entry with timestamp, content, and type
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub content: String,
    pub log_type: LogType,
}

impl LogEntry {
    pub fn new(content: String, log_type: LogType) -> Self {
        let timestamp = chrono::Local::now().format("[%H:%M:%S]").to_string();
        Self {
            timestamp,
            content,
            log_type,
        }
    }

    pub fn new_with_timestamp(content: String, log_type: LogType, timestamp: String) -> Self {
        Self {
            timestamp,
            content,
            log_type,
        }
    }

    pub fn command(content: String) -> Self {
        Self::new(content, LogType::Command)
    }

    pub fn output(content: String) -> Self {
        Self::new(content, LogType::Output)
    }

    pub fn error(content: String) -> Self {
        Self::new(content, LogType::Error)
    }

    pub fn info(content: String) -> Self {
        Self::new(content, LogType::Info)
    }

    pub fn warning(content: String) -> Self {
        Self::new(content, LogType::Warning)
    }

    /// Format the log entry for display with colors
    pub fn format_for_display(&self, ui: &egui::Ui) -> RichText {
        let formatted = format!("{} {}", self.timestamp, self.content);

        RichText::new(formatted)
            .color(self.log_type.color(ui))
            .monospace()
    }
}

// Tab types for the dock interface
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Tab {
    Source,
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
            Self::Source => "Source",
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Tabs {
    scroll_lock: bool,

    #[serde(skip)]
    console_input: String,
    #[serde(skip)]
    console_input_prev: String,
    #[serde(skip)]
    logs: Vec<LogEntry>,
    #[serde(skip)]
    gdb_available: bool,
    #[serde(skip)]
    pending_commands: Vec<String>,
    #[serde(skip)]
    last_log_count: usize,
    #[serde(skip)]
    focused_line: Option<usize>,
}

impl Tabs {
    /// Set the focused line (1-based). Pass `None` to clear focus
    pub fn set_focused_line(&mut self, line_number: Option<usize>) {
        self.focused_line = line_number;
    }

    /// Update logs with GDB output
    pub fn update_from_gdb(&mut self, gdb: &Gdb) {
        while let Some(output) = gdb.try_receive_output() {
            // Determine log type based on content
            let log_type = if output.contains("error") || output.contains("Error") {
                LogType::Error
            } else if output.contains("warning") || output.contains("Warning") {
                LogType::Warning
            } else if output.contains("info") || output.contains("Info") {
                LogType::Info
            } else {
                LogType::Output
            };

            self.logs.push(LogEntry::new(output, log_type));
        }
    }

    /// Send a command to GDB and add it to logs
    pub fn send_command_to_gdb(
        &mut self,
        command: &str,
        gdb: &Gdb,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.logs.push(LogEntry::command(command.to_owned()));

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

    /// Add a custom log entry
    pub fn add_log_entry(&mut self, log_entry: LogEntry) {
        self.logs.push(log_entry);
    }

    /// Add a log entry with custom timestamp
    pub fn add_log_with_timestamp(
        &mut self,
        content: String,
        log_type: LogType,
        timestamp: String,
    ) {
        self.logs
            .push(LogEntry::new_with_timestamp(content, log_type, timestamp));
    }

    /// Add an info message
    pub fn add_info(&mut self, message: String) {
        self.logs.push(LogEntry::info(message));
    }

    /// Add an error message
    pub fn add_error(&mut self, message: String) {
        self.logs.push(LogEntry::error(message));
    }

    /// Add a warning message
    pub fn add_warning(&mut self, message: String) {
        self.logs.push(LogEntry::warning(message));
    }

    /// Clear all logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Get the number of log entries
    pub fn log_count(&self) -> usize {
        self.logs.len()
    }
}

fn code_with_line_numbers(
    ui: &mut Ui,
    code: &str,
    language: &str,
    theme: &CodeTheme,
    focused_line: Option<usize>,
) {
    // HACK: Add more code for testing
    // let mut sb = String::from(code);
    // for _ in 0..100 {
    //     sb.push_str(code);
    // }
    // let code = &sb.to_string();

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let focus_fill = if ui.visuals().dark_mode {
                Color32::from_rgba_unmultiplied(60, 90, 160, 80)
            } else {
                Color32::from_rgba_unmultiplied(100, 140, 220, 60)
            };

            for (i, line) in code.lines().enumerate() {
                let is_focused = focused_line.is_some_and(|ln| ln == i + 1);

                if is_focused {
                    let response = egui::Frame::new()
                        .fill(focus_fill)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let line_num = format!("{:>3} ", i + 1);
                                ui.label(RichText::new(line_num).monospace().weak());

                                let highlighted =
                                    highlight(ui.ctx(), ui.style(), theme, line, language);
                                ui.label(highlighted);
                            });
                        })
                        .response;

                    ui.scroll_to_rect(response.rect, Some(egui::Align::Center));
                } else {
                    ui.horizontal(|ui| {
                        let line_num = format!("{:>3} ", i + 1);
                        ui.label(RichText::new(line_num).monospace().weak());

                        let highlighted = highlight(ui.ctx(), ui.style(), theme, line, language);
                        ui.label(highlighted);
                    });
                }
            }
        });
}

impl TabViewer for Tabs {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Source => {
                self.set_focused_line(Some(40));
                let code = r#"#include <stdio.h>

int main(int argc, char **argv) {
    for (size_t i = 0;; ++i) {
        printf("%zu\n", i);
    }

    return 0;
}"#;
                let language = "c";
                let theme =
                    &egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
                code_with_line_numbers(ui, code, language, theme, self.focused_line);
            }
            Tab::Console => {
                ui.vertical(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), ui.available_height() - 30.0),
                        egui::Layout::top_down(egui::Align::default()),
                        |ui| {
                            let mut scroll_area = ScrollArea::new([true, true]).auto_shrink(false);

                            let should_auto_scroll =
                                self.scroll_lock && self.logs.len() > self.last_log_count;

                            if should_auto_scroll {
                                // TODO: Fix scroll offset
                                scroll_area = scroll_area.vertical_scroll_offset(999999.0);
                            }

                            scroll_area.show(ui, |ui| {
                                for log_entry in &self.logs {
                                    ui.label(log_entry.format_for_display(ui));
                                }
                            });

                            if should_auto_scroll {
                                self.last_log_count = self.logs.len();
                            }
                        },
                    );

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Command");

                        // Reserve space for the controls at the end
                        // TODO: Fix magic so it's dynamic
                        let controls_width = ui.available_width() - 105.0;
                        let response = ui.add_sized(
                            [controls_width, ui.available_height()],
                            TextEdit::singleline(&mut self.console_input)
                                .font(TextStyle::Monospace),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
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

                        ui.checkbox(&mut self.scroll_lock, "Scroll");

                        if ui.button("Clear").clicked() {
                            self.clear_logs();
                        }
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
