use std::path::PathBuf;

use eframe::egui;

use crate::app::App;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewProjectModalWindow {
    pub name: String,
    pub color: egui::Color32,
    pub path: PathBuf,
}

impl Default for NewProjectModalWindow {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: egui::Color32::BLUE,
            path: PathBuf::new(),
        }
    }
}
pub fn draw_new_project_modal_window(
    ui: &egui::Ui,
    app: &mut App,
    new_project_modal_window: &mut NewProjectModalWindow,
) {
    egui::Modal::new(egui::Id::new("new_project_modal")).show(ui, |ui| {
        ui.heading("New project");

        ui.label("Project name :");
        let response = ui.text_edit_singleline(&mut new_project_modal_window.name);

        ui.label("Couleur :");
        ui.color_edit_button_srgba(&mut new_project_modal_window.color);

        ui.separator();

        ui.horizontal(|ui: &mut egui::Ui| {
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if ui.button("Créer").clicked() || enter {
                app.user_created_project(new_project_modal_window);
                app.state.modal_window = super::modal_window::ModalWindow::None;
            }
            if ui.button("Annuler").clicked() {
                app.state.modal_window = super::modal_window::ModalWindow::None;
            }
        });
    });
}
