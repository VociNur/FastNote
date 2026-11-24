use std::path::PathBuf;

use eframe::egui::{self, Color32};

use crate::{
    app::App, folder_exists, is_valid_folder_name, projects::fastnote_project::FastnoteFolder,
    ui::modal_windows::modal_window::ModalWindow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFolderModalWindow {
    pub parent: PathBuf,
    pub folder_name: String,
    pub display_name: String,
}

impl Default for NewFolderModalWindow {
    fn default() -> Self {
        Self {
            folder_name: String::new(),
            display_name: String::new(),
            parent: PathBuf::new(),
        }
    }
}

impl NewFolderModalWindow {
    pub fn new(parent: PathBuf) -> Self {
        NewFolderModalWindow {
            folder_name: "".to_owned(),
            display_name: "".to_owned(),
            parent,
        }
    }
}

pub fn draw_new_folder_modal_window(
    ui: &egui::Ui,
    app: &mut App,
    modal: &mut NewFolderModalWindow,
) {
    egui::Modal::new(egui::Id::new("new_folder_modal")).show(ui, |ui| {
        ui.heading("Nouveau dossier");

        if modal.folder_name.is_empty() {
            //osef pas d’erreur ici
        } else if !is_valid_folder_name(&modal.folder_name) {
            // draw_error_banner(ui, "Caractère non valide dans le nom de fichier");
            app.push_instant_error("Caractère non valide dans le nom de fichier");
        } else if folder_exists(&modal.parent, &modal.folder_name) {
            // draw_error_banner(ui, "Un fichier portant ce nom existe déjà");
            app.push_instant_error("Un fichier portant ce nom existe déjà");
        }
        ui.label("Nom du dossier :");
        let response = ui.text_edit_singleline(&mut modal.folder_name);

        ui.separator();

        ui.horizontal(|ui| {
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if ui.button("Créer").clicked() || enter {
                let folder_path = modal.parent.join(&modal.folder_name);

                std::fs::create_dir_all(&folder_path).ok();

                // Création manifest
                let folder_response = FastnoteFolder::create_blank(
                    folder_path,
                    modal.folder_name.clone(),
                    Color32::BLUE,
                );
                match folder_response {
                    Ok(file) => {
                        let save_response = file.save();
                        if let Err(err) = save_response {
                            app.push_instant_error(err.to_string());
                        }
                    }
                    Err(err) => {
                        app.push_instant_error(err.to_string());
                    }
                }
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
