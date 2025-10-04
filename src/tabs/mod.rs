use egui::{Ui, WidgetText};
use egui_dock::TabViewer;
use serde::{Deserialize, Serialize};

// Trait for tab implementations
pub trait TabImpl {
    fn title(&self) -> &'static str;
    fn ui(&mut self, ui: &mut Ui, app: &mut crate::app::Gdbr);
}

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

// Main tabs container - now stateless
#[derive(Debug, Default)]
pub struct Tabs;

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
                ui.centered_and_justified(|ui| ui.heading("Console"));
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

// Export individual tab modules
pub mod content;
pub mod console;
pub mod exe;
pub mod breakpoints;
pub mod commands;
pub mod struct_tab;
pub mod stack;
pub mod files;
pub mod thread;
pub mod cmd_search;
pub mod watch;
pub mod locals;
pub mod registers;
pub mod data;
