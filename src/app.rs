use egui::{
    Color32, MenuBar, ScrollArea, Slider, TextEdit, TextStyle, ThemePreference, TopBottomPanel, Ui,
    WidgetText,
};
use egui_dock::{DockArea, DockState, Style, TabViewer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const NAME: &str = "gdbr";

struct Tabs;

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
                ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Console");
                                ui.add_sized(
                                    ui.available_size(),
                                    TextEdit::singleline(&mut String::new())
                                        .font(TextStyle::Monospace),
                                );
                            });

                            ui.separator();

                            ScrollArea::new([true, true])
                                .auto_shrink(false)
                                .show(ui, |ui| {
                                    ui.add_sized(
                                        ui.available_size(),
                                        TextEdit::multiline(&mut "Logs...\nMore logs...\nSome other log...\nNew log...\nAnother log".to_owned())
                                            .font(TextStyle::Monospace),
                                    );
                                });
                        });
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

// Tab types for the dock interface
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    Content,
    Console,

    Exe,
    Breakpoints,
    Commands,
    Struct,

    Watch,
    Locals,
    Registers,
    Data,
}

impl Tab {
    fn title(&self) -> &'static str {
        match self {
            Self::Content => "Content",
            Self::Console => "Console",

            Self::Exe => "Exe",
            Self::Breakpoints => "Breakpoints",
            Self::Commands => "Commands",
            Self::Struct => "Struct",

            Self::Watch => "Watch",
            Self::Locals => "Locals",
            Self::Registers => "Registers",
            Self::Data => "Data",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Gdbr {
    zoom: f32,
    #[serde(skip)]
    zoom_temp: f32,
    #[serde(skip)]
    logs: String, // TODO: vec
    #[serde(skip)]
    console_input: String,
    #[serde(skip)] // TODO: remove
    dock_state: DockState<Tab>,
    #[serde(skip)]
    layout_initialized: bool,
}

impl Default for Gdbr {
    fn default() -> Self {
        // Start with a basic layout - we'll enhance it in setup_dock_layout
        let initial_tabs = vec![Tab::Watch];

        Self {
            zoom: 1.0,
            zoom_temp: 1.0,
            logs: "Logs...\nMore logs...\nSome other log...\nNew log...\nAnother log".into(),
            console_input: String::new(),
            dock_state: DockState::new(initial_tabs),
            layout_initialized: false,
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
            let mut app: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            // Setup temps
            app.zoom_temp = app.zoom;

            // Initialize dock state if it wasn't loaded from storage
            if app.dock_state.main_surface().is_empty() {
                app.dock_state = DockState::new(vec![Tab::Watch]);
                app.layout_initialized = false;
            }

            app
        } else {
            Default::default()
        }
    }

    fn show_dock_area(&mut self, ctx: &egui::Context) {
        self.setup_dock_layout();

        let mut tab_viewer = Tabs;
        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut tab_viewer);
    }

    fn setup_dock_layout(&mut self) {
        // Only set up the layout once, when the app starts
        if !self.layout_initialized {
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

            // Mark layout as initialized so we don't recreate it
            self.layout_initialized = true;
        }
    }
}

impl eframe::App for Gdbr {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(self.zoom);

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

                            ui.add(Slider::new(&mut self.zoom_temp, 0.5..=3.0));

                            if ui.button("Apply").clicked() {
                                self.zoom = self.zoom_temp;
                            }
                        });
                    });
                });
            });
        });

        self.show_dock_area(ctx);
    }
}
