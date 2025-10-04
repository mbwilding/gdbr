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

#[derive(Debug)]
pub struct Tabs {
    console_input: String,
    logs: String,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            console_input: Default::default(),
            logs: String::from(
                r#"Log entry 1: Starting application.
Log entry 2: Initializing configuration.
Log entry 3: Configuration loaded successfully.
Log entry 4: Connecting to database.
Log entry 5: Database connection established.
Log entry 6: Checking for pending migrations.
Log entry 7: No migrations needed.
Log entry 8: Spawning worker threads.
Log entry 9: Worker thread 1 started.
Log entry 10: Worker thread 2 started.
Log entry 11: Worker thread 3 started.
Log entry 12: HTTP server listening on port 8080.
Log entry 13: Received incoming connection from 192.168.0.2.
Log entry 14: Performing user authentication.
Log entry 15: User authentication successful.
Log entry 16: Processing API request for /v1/data.
Log entry 17: Querying database for data.
Log entry 18: Data retrieval successful.
Log entry 19: Responding to client request.
Log entry 20: Received another connection from 192.168.0.3.
Log entry 21: User authentication failed for user guest.
Log entry 22: Closing connection due to failed authentication.
Log entry 23: Received signal SIGHUP, reloading configuration.
Log entry 24: Configuration reload complete.
Log entry 25: Periodic cleanup started.
Log entry 26: Removed 124 expired sessions.
Log entry 27: Cleanup complete.
Log entry 28: Detecting system resources.
Log entry 29: Memory usage is within normal parameters.
Log entry 30: Disk space check passed.
Log entry 31: Received shutdown request.
Log entry 32: Notifying worker threads to stop.
Log entry 33: Worker thread 1 shutting down.
Log entry 34: Worker thread 2 shutting down.
Log entry 35: Worker thread 3 shutting down.
Log entry 36: All worker threads stopped.
Log entry 37: Shutting down HTTP server.
Log entry 38: HTTP server stopped.
Log entry 39: Disconnecting from database.
Log entry 40: Database connection closed successfully.
Log entry 41: Application shutdown complete.
Log entry 42: Starting backup process.
Log entry 43: Backup completed successfully.
Log entry 44: Monitoring services restarted.
Log entry 45: Service heartbeat received.
Log entry 46: Service heartbeat missed, attempting recovery.
Log entry 47: Recovery process succeeded.
Log entry 48: Performing log rotation.
Log entry 49: Old logs archived.
Log entry 50: Log rotation complete.
"#,
            ),
        }
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
                        ui.add_sized(
                            ui.available_size(),
                            TextEdit::singleline(&mut self.console_input)
                                .font(TextStyle::Monospace),
                        );
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
