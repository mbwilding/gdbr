use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::ui::UiManager;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    config: Config,
    ui: UiManager,
}

impl Gdbr {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        // Setup temps
        app.ui.zoom_temp = app.ui.zoom;

        // Setup UI
        app.ui.setup_theme(&cc.egui_ctx);
        app.ui.setup_fonts(&cc.egui_ctx);

        app
    }
}

impl eframe::App for Gdbr {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.ui.zoom);

        self.ui.show_menu_bar(ctx);
        self.ui.show_dock_area(ctx);
    }
}
