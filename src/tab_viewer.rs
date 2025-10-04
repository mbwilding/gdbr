#[derive(Debug, Default)]
pub struct TabViewer {
    modified: bool,
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {tab}"));
    }

    fn on_rect_changed(&mut self, _tab: &mut Self::Tab) {
        self.modified = true;
    }
}
