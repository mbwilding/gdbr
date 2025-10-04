use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::ui::UiManager;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    config: AppConfig,
    #[serde(skip)]
    ui_manager: Option<UiManager>,
}

impl Default for Gdbr {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
            ui_manager: None,
        }
    }
}

impl Gdbr {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        };

        // Setup temps
        app.config.zoom_temp = app.config.zoom;

        // Initialize dock state if it wasn't loaded from storage
        if app.ui_manager.is_none() {
            app.ui_manager = Some(UiManager::new(app.config.clone()));
        }

        // Setup theme and fonts
        if let Some(ref ui_manager) = app.ui_manager {
            ui_manager.setup_theme(&cc.egui_ctx);
            ui_manager.setup_fonts(&cc.egui_ctx);
        }

        app
    }
}

impl eframe::App for Gdbr {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.config.zoom);

        if let Some(ref mut ui_manager) = self.ui_manager {
            ui_manager.show_menu_bar(ctx);
            ui_manager.show_dock_area(ctx);
        }
    }
}
