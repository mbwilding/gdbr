use crate::{cli::Cli, gdb::Gdb, ui::UiManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    #[serde(skip)]
    cli: Cli,

    ui: UiManager,

    #[serde(skip)]
    gdb: Option<Gdb>,
}

impl Gdbr {
    pub fn new(cc: &eframe::CreationContext<'_>, _cli: Option<Cli>) -> Self {
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
        self.ui.update(ctx);
        self.ui.show_menu_bar(ctx);
        self.ui.show_dock_area(ctx);
    }
}
