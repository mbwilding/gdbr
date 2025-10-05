use std::{error::Error, path::Path};

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
    pub fn new(cc: &eframe::CreationContext<'_>, cli: Option<Cli>) -> Self {
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

        if let Some(cli) = cli
            && let Some(executable) = cli.executable
        {
            // This will start the gdb process on first update call
            app.ui.set_file_details(Path::new(&executable));
        }

        app
    }

    /// Spawn a new GDB process for the given file
    pub fn spawn_gdb(&mut self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut args = self.cli.gdb_args.clone();
        args.push(file_path.to_string_lossy().to_string());
        self.gdb = Some(Gdb::new(args)?);
        Ok(())
    }

    /// Get a reference to the GDB instance if it exists
    pub fn gdb(&self) -> Option<&Gdb> {
        self.gdb.as_ref()
    }

    /// Get a mutable reference to the GDB instance if it exists
    pub fn gdb_mut(&mut self) -> Option<&mut Gdb> {
        self.gdb.as_mut()
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

        // Check if a file was just loaded and spawn GDB if needed
        if self.ui.was_file_just_loaded()
            && let Some(file_path) = self.ui.get_picked_file()
        {
            let file_path = file_path.clone();
            if let Err(e) = self.spawn_gdb(&file_path) {
                eprintln!("Failed to spawn GDB: {e}");
                self.ui.add_error(format!("Failed to spawn GDB: {e}"));
            } else {
                self.ui.set_gdb_available(true);
                self.ui
                    .add_info("GDB process started successfully".to_owned());
            }
        }

        if let Some(gdb) = &self.gdb {
            self.ui.update_from_gdb(gdb);

            if let Err(e) = self.ui.process_pending_commands(gdb) {
                eprintln!("Failed to process pending commands: {e}");
                self.ui
                    .add_error(format!("Failed to process pending commands: {e}"));
            }
        }
    }
}
