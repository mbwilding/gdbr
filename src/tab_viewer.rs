use egui_dock::{DockState, NodeIndex};

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

impl TabViewer {
    pub fn default_layout() -> DockState<String> {
        let mut tree = DockState::new(vec!["Central".to_owned()]);

        let [_central, _right] =
            tree.main_surface_mut()
                .split_left(NodeIndex::root(), 0.80, vec!["Right".to_owned()]);

        let [_, _bottom] =
            tree.main_surface_mut()
                .split_below(NodeIndex::root(), 0.65, vec!["Bottom".to_owned()]);

        tree
    }
}
