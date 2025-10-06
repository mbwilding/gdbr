use crate::gdb::Gdb;
use crate::tabs::{Tab, Tabs};
use egui::{Color32, MenuBar, RichText, TopBottomPanel};
use egui_dock::{DockArea, DockState, Style};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UiManager {
    dock_state: DockState<Tab>,
    tabs: Tabs,

    #[serde(skip)]
    picked_file: Option<PathBuf>,
    #[serde(skip)]
    picked_path_file_name: String,
    #[serde(skip)]
    picked_path_full_path: String,

    #[serde(skip)]
    file_just_loaded: bool,

    pub zoom: f32,
    #[serde(skip)]
    pub zoom_temp: f32,
}

impl Default for UiManager {
    fn default() -> Self {
        Self {
            dock_state: Self::setup_dock_layout(),
            tabs: Tabs::default(),

            picked_file: None,
            picked_path_file_name: String::new(),
            picked_path_full_path: String::new(),

            file_just_loaded: false,

            zoom: 1.0,
            zoom_temp: 1.0,
        }
    }
}

impl UiManager {
    pub fn setup_theme(&self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals {
            panel_fill: Color32::BLACK,
            ..Default::default()
        });
    }

    pub fn setup_fonts(&self, ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "NeoSpleen".to_owned(),
            #[expect(clippy::large_include_file)]
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/NeoSpleenNerdFont-Regular.ttf"
            ))),
        );

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "NeoSpleen".to_owned());

        ctx.set_fonts(fonts);
    }

    pub fn show_menu_bar(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked()
                        && let Some(path) = rfd::FileDialog::new().pick_file()
                    {
                        self.set_file_details(&path);
                    }

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Windows", |ui| {
                    let tabs = [
                        Tab::Source,
                        Tab::Console,
                        Tab::Exe,
                        Tab::Breakpoints,
                        Tab::Commands,
                        Tab::Struct,
                        Tab::Stack,
                        Tab::Files,
                        Tab::Thread,
                        Tab::CmdSearch,
                        Tab::Watch,
                        Tab::Locals,
                        Tab::Registers,
                        Tab::Data,
                    ];

                    for tab in tabs {
                        let is_visible = self.is_tab_visible(&tab);
                        let mut visible = is_visible;

                        if ui.checkbox(&mut visible, tab.title()).clicked() {
                            self.set_tab_visible(tab, visible);
                        }
                    }

                    ui.separator();

                    if ui.button("Reset Layout").clicked() {
                        self.dock_state = Self::setup_dock_layout();
                    }
                });

                if self.picked_file.is_some() {
                    ui.centered_and_justified(|ui| {
                        ui.label(&self.picked_path_file_name)
                            .on_hover_text(&self.picked_path_full_path);
                    });
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if cfg!(debug_assertions) {
                        ui.label(
                            RichText::new("Debug")
                                .small()
                                .color(ui.visuals().warn_fg_color),
                        )
                        .on_hover_text("Debug build");
                    }

                    egui::widgets::global_theme_preference_switch(ui);

                    let zoom_response = ui.add(
                        egui::DragValue::new(&mut self.zoom_temp)
                            .update_while_editing(false)
                            .speed(0.1)
                            .range(1.0..=3.0)
                            .prefix("Zoom: "),
                    );

                    if zoom_response.lost_focus() || zoom_response.drag_stopped() {
                        self.zoom = self.zoom_temp;
                    }
                });
            });
        });
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if let Some(file) = i.raw.dropped_files.first()
                && let Some(path) = &file.path
            {
                self.set_file_details(path);
            }
        });
    }

    pub fn set_file_details(&mut self, path: &Path) {
        self.picked_file = Some(path.to_path_buf());
        self.picked_path_full_path = path.display().to_string();
        if let Some(file_name) = path.file_name() {
            self.picked_path_file_name = file_name.display().to_string();
        }
        self.file_just_loaded = true;
    }

    /// Get the currently picked file path
    pub fn get_picked_file(&self) -> Option<&PathBuf> {
        self.picked_file.as_ref()
    }

    /// Check if a file was just loaded and reset the flag
    pub fn was_file_just_loaded(&mut self) -> bool {
        if self.file_just_loaded {
            self.file_just_loaded = false;
            true
        } else {
            false
        }
    }

    pub fn show_dock_area(&mut self, ctx: &egui::Context) {
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut self.tabs);
    }

    /// Update tabs with GDB output if GDB is available
    pub fn update_from_gdb(&mut self, gdb: &Gdb) {
        self.tabs.update_from_gdb(gdb);
    }

    /// Send a command to GDB through tabs
    pub fn send_command_to_gdb(&mut self, command: &str, gdb: &Gdb) -> Result<(), Box<dyn Error>> {
        self.tabs.send_command_to_gdb(command, gdb)
    }

    /// Process pending commands and send them to GDB
    pub fn process_pending_commands(&mut self, gdb: &Gdb) -> Result<(), Box<dyn Error>> {
        let commands = self.tabs.take_pending_commands();
        for command in commands {
            self.send_command_to_gdb(&command, gdb)?;
        }
        Ok(())
    }

    /// Set GDB availability in tabs
    pub fn set_gdb_available(&mut self, available: bool) {
        self.tabs.set_gdb_available(available);
    }

    /// Add an info message to the console
    pub fn add_info(&mut self, message: String) {
        self.tabs.add_info(message);
    }

    /// Add an error message to the console
    pub fn add_error(&mut self, message: String) {
        self.tabs.add_error(message);
    }

    /// Add a warning message to the console
    pub fn add_warning(&mut self, message: String) {
        self.tabs.add_warning(message);
    }

    /// Clear all console logs
    pub fn clear_logs(&mut self) {
        self.tabs.clear_logs();
    }

    pub fn is_tab_visible(&self, tab: &Tab) -> bool {
        self.dock_state.iter_all_tabs().any(|(_, t)| t == tab)
    }

    pub fn toggle_tab(&mut self, tab: Tab) {
        if self.is_tab_visible(&tab) {
            if let Some((surface, node, tab_index)) = self.dock_state.find_tab(&tab) {
                self.dock_state.remove_tab((surface, node, tab_index));
            }
        } else {
            self.dock_state.push_to_focused_leaf(tab);
        }
    }

    pub fn set_tab_visible(&mut self, tab: Tab, visible: bool) {
        if visible && !self.is_tab_visible(&tab) {
            self.dock_state.push_to_focused_leaf(tab);
        } else if !visible
            && self.is_tab_visible(&tab)
            && let Some((surface, node, tab_index)) = self.dock_state.find_tab(&tab)
        {
            self.dock_state.remove_tab((surface, node, tab_index));
        }
    }

    fn setup_dock_layout() -> DockState<Tab> {
        let mut dock_state = DockState::<Tab>::new(vec![Tab::Source]);
        let surface = dock_state.main_surface_mut();

        let [center, bottom_left] =
            surface.split_below(egui_dock::NodeIndex::root(), 0.6666666, vec![Tab::Console]);

        let [_, _bottom_right] = surface.split_right(
            bottom_left,
            0.6666666,
            vec![Tab::Watch, Tab::Locals, Tab::Registers, Tab::Data],
        );

        let [_, right_top] = surface.split_right(
            center,
            0.8,
            vec![Tab::Exe, Tab::Breakpoints, Tab::Commands, Tab::Struct],
        );

        let [_, _right_bottom] = surface.split_below(
            right_top,
            0.5,
            vec![Tab::Stack, Tab::Files, Tab::Thread, Tab::CmdSearch],
        );

        dock_state
    }
}
