use crate::tab_viewer::TabViewer;
use egui::Frame;
use egui::{CentralPanel, Color32, MenuBar, TopBottomPanel};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    pub dock_state: DockState<String>,
}

pub fn reset_dock() -> DockState<String> {
    let mut dock_state = DockState::new(vec!["tab1".to_owned(), "tab2".to_owned()]);

    let [left, right] =
        dock_state
            .main_surface_mut()
            .split_left(NodeIndex::root(), 0.3, vec!["tab3".to_owned()]);

    let [_, _] = dock_state
        .main_surface_mut()
        .split_below(left, 0.7, vec!["tab4".to_owned()]);

    let [_, _] = dock_state
        .main_surface_mut()
        .split_below(right, 0.5, vec!["tab5".to_owned()]);

    dock_state
}

impl Default for Gdbr {
    fn default() -> Self {
        Self {
            dock_state: reset_dock(),
        }
    }
}

impl Gdbr {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            // Foreground color
            override_text_color: Some(Color32::WHITE),
            // Background color
            panel_fill: Color32::BLACK,
            // Button frame
            button_frame: true,
            ..Default::default()
        });

        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "NeoSpleen".to_owned(),
            #[expect(clippy::large_include_file)]
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../assets/NeoSpleenNerdFont-Regular.ttf"
            ))),
        );

        // fonts
        //     .families
        //     .entry(egui::FontFamily::Proportional)
        //     .or_default()
        //     .insert(0, "NeoSpleen".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "NeoSpleen".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}
impl eframe::App for Gdbr {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Tabs
        let mut tab_viewer = TabViewer::default();
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);

        // Top bar
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    if ui.button("Reset Dock").clicked() {
                        self.dock_state = reset_dock();
                    }
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // Central tab viewer
        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                DockArea::new(&mut self.dock_state).show_inside(ui, &mut tab_viewer);
            });
    }
}
