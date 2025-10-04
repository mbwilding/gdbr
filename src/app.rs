use egui::{CentralPanel, Color32, MenuBar, ScrollArea, TextEdit, ThemePreference, TopBottomPanel};
use egui::{Frame, SidePanel};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const NAME: &'static str = "gdbr";

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    #[serde(skip)]
    logs: String, // TODO: Vec
    #[serde(skip)]
    console_input: String,
}

impl Default for Gdbr {
    fn default() -> Self {
        Self {
            logs: "Logs...\nMore logs...\nSome other log...\nNew log...\nAnother log".into(),
            console_input: String::new(),
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
        TopBottomPanel::top("top").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Settings", |ui| {
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
                });

                // ui.add_space(16.0);
            });
        });

        TopBottomPanel::bottom("bottom")
            .resizable(true)
            .default_height(350.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                SidePanel::right("right2")
                    .resizable(true)
                    .default_width(650.0)
                    .min_width(100.0)
                    .show_inside(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.heading("Watch | Locals | Registers | Data")
                        })
                    });

                ScrollArea::new([true, true])
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.add_sized(ui.available_size(), TextEdit::multiline(&mut self.logs));
                    });

                // ui.label("Console")

                // ui.centered_and_justified(|ui| ui.label("Console"));
                // // ui.text_edit_multiline(&mut self.logs);
                //
                // ui.horizontal(|ui| {
                //     ui.text_edit_singleline(&mut self.console_input);
                // });
            });

        SidePanel::right("right")
            .resizable(true)
            .default_width(500.0)
            .min_width(100.0)
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| ui.heading("Right"))
            });

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| ui.heading("Center"))
            });
    }
}
