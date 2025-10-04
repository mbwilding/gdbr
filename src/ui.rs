use egui::{Color32, MenuBar, RichText, Slider, TopBottomPanel};
use egui_dock::{DockArea, DockState, Style};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::config::Config;
use crate::tabs::{Tab, Tabs};

#[derive(Debug, Deserialize, Serialize)]
pub struct UiManager {
    config: Config,
    dock_state: DockState<Tab>,

    pub zoom: f32,
    #[serde(skip)]
    pub zoom_temp: f32,
}

impl Default for UiManager {
    fn default() -> Self {
        Self {
            config: Default::default(),
            dock_state: Self::setup_dock_layout(),

            zoom: 1.0,
            zoom_temp: 1.0,
        }
    }
}

impl UiManager {
    pub fn setup_theme(&self, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            override_text_color: Some(Color32::WHITE),
            panel_fill: Color32::BLACK,
            button_frame: true,
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
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Windows", |ui| {
                    let tabs = [
                        Tab::Content,
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

                    if ui.button("Apply").clicked() {
                        self.zoom = self.zoom_temp;
                    }
                    ui.add(
                        Slider::new(&mut self.zoom_temp, 0.5..=2.0)
                            .step_by(0.1)
                            .fixed_decimals(1)
                            .text("Zoom"),
                    );
                });
            });
        });
    }

    pub fn show_dock_area(&mut self, ctx: &egui::Context) {
        let mut tab_viewer = Tabs;
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
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
        let mut dock_state = DockState::<Tab>::new(vec![Tab::Content]);
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
