use std::path::PathBuf;

use eframe::egui;

// Dans ton state
pub struct NewProjectDialog {
    pub open:  bool,
    pub name:  String,
    pub color: egui::Color32,
    pub path:  PathBuf,
}

impl Default for NewProjectDialog {
    fn default() -> Self {
        Self {
            open:  false,
            name:  String::new(),
            color: egui::Color32::BLUE,
            path:  PathBuf::new(),
        }
    }
}
