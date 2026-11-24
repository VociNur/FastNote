use std::path::PathBuf;

use eframe::egui;

use crate::{app::App, ui::modal_windows::modal_window::ModalWindow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameModalWindow {
    pub old_path: PathBuf,
    pub new_name: String,
}

impl Default for RenameModalWindow {
    fn default() -> Self {
        Self {
            old_path: PathBuf::new(),
            new_name: String::new(),
        }
    }
}

impl RenameModalWindow {
    pub fn new(old_path: PathBuf) -> Self {
        Self {
            old_path,
            new_name: "".to_owned(),
        }
    }
}

pub fn draw_rename_modal_window(ui: &egui::Ui, app: &mut App, modal: &mut RenameModalWindow) {
    egui::Modal::new(egui::Id::new("rename_modal")).show(ui, |ui| {
        ui.heading("Renommer");

        ui.label("Nouveau nom :");
        let response = ui.text_edit_singleline(&mut modal.new_name);

        ui.separator();

        ui.horizontal(|ui| {
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if ui.button("Renommer").clicked() || enter {
                let new_path = modal.old_path.parent().unwrap().join(&modal.new_name);

                std::fs::rename(&modal.old_path, &new_path).ok();

                // Reload du projet
                app.reload_current_project();

                app.state.modal_window = ModalWindow::None;
            }

            if ui.button("Annuler").clicked() {
                app.state.modal_window = ModalWindow::None;
            }
        });
    });
}
