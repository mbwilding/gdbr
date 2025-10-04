use egui::{Color32, MenuBar, Slider, ThemePreference, TopBottomPanel};
use egui_dock::{DockArea, DockState, Style};
use std::sync::Arc;

use crate::config::AppConfig;
use crate::tabs::{Tab, Tabs};

#[derive(Debug)]
pub struct UiManager {
    config: AppConfig,
    dock_state: DockState<Tab>,
}

impl UiManager {
    pub fn new(config: AppConfig) -> Self {
        let initial_tabs = vec![Tab::Content];
        Self {
            config,
            dock_state: DockState::new(initial_tabs),
        }
    }

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

                ui.menu_button("Settings", |ui| {
                    ui.menu_button("Visuals", |ui| {
                        ui.menu_button("Themes", |ui| {
                            if ui.button("System").clicked() {
                                ctx.set_theme(ThemePreference::System);
                            }
                            if ui.button("Dark").clicked() {
                                ctx.set_theme(ThemePreference::Dark);
                            }
                            if ui.button("Light").clicked() {
                                ctx.set_theme(ThemePreference::Light);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Zoom ");
                            ui.add(Slider::new(&mut self.config.zoom_temp, 0.5..=3.0));
                            if ui.button("Apply").clicked() {
                                self.config.zoom = self.config.zoom_temp;
                            }
                        });
                    });
                });
            });
        });
    }

    pub fn show_dock_area(&mut self, ctx: &egui::Context) {
        self.setup_dock_layout();

        let mut tab_viewer = Tabs;
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    fn setup_dock_layout(&mut self) {
        if !self.config.layout_initialized {
            let surface = self.dock_state.main_surface_mut();

            // Create a proper split layout:
            // 1. Split horizontally: left (debug area) and right (content)
            let content_tabs = vec![Tab::Content];
            let [_left, right_node] =
                surface.split_right(egui_dock::NodeIndex::root(), 0.4, content_tabs);

            // 2. Split the left side vertically: top (debug tabs) and bottom (console)
            let debug_tabs = vec![Tab::Locals, Tab::Registers, Tab::Data];
            let [_top, bottom_node] =
                surface.split_below(egui_dock::NodeIndex::root(), 0.7, debug_tabs);

            // 3. Add console at the bottom
            let console_tabs = vec![Tab::Console];
            surface.split_below(bottom_node, 0.3, console_tabs);

            // 4. Add additional tabs to the right side
            let additional_tabs = vec![Tab::Exe, Tab::Breakpoints, Tab::Commands, Tab::Struct];
            surface.split_below(right_node, 0.5, additional_tabs);

            self.config.layout_initialized = true;
        }
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }
}
