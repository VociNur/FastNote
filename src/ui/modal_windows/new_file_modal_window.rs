use std::path::PathBuf;

use eframe::egui::{self, Color32};

use crate::{
    app::App, folder_exists, is_valid_folder_name, projects::fastnote_project::FastnoteFile,
    ui::modal_windows::modal_window::ModalWindow,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFileModalWindow {
    pub folder_name: String,  // nom réel du dossier
    pub display_name: String, // nom personnalisé
    // pub color: egui::Color32,
    pub parent: PathBuf,
}

impl Default for NewFileModalWindow {
    fn default() -> Self {
        Self {
            folder_name: String::new(),
            display_name: String::new(),
            parent: PathBuf::new(),
        }
    }
}

impl NewFileModalWindow {
    pub fn new(parent: PathBuf) -> Self {
        NewFileModalWindow {
            folder_name: "".to_owned(),
            display_name: "".to_owned(),
            parent,
        }
    }
}

pub fn draw_new_file_modal_window(ui: &egui::Ui, app: &mut App, modal: &mut NewFileModalWindow) {
    egui::Modal::new(egui::Id::new("new_file_modal")).show(ui, |ui| {
        ui.heading("Nouveau fichier");
        if modal.folder_name.is_empty() {
            //osef pas d’erreur ici
        } else if !is_valid_folder_name(&modal.folder_name) {
            // draw_error_banner(ui, "Caractère non valide dans le nom de fichier");
            app.push_instant_error("Caractère non valide dans le nom de fichier");
        } else if folder_exists(&modal.parent, &modal.folder_name) {
            // draw_error_banner(ui, "Un fichier portant ce nom existe déjà");
            app.push_instant_error("Un fichier portant ce nom existe déjà");
        }
        ui.label("Nom du dossier (obligatoire) :");
        ui.text_edit_singleline(&mut modal.folder_name);

        ui.label("Nom personnalisé (facultatif) :");
        ui.text_edit_singleline(&mut modal.display_name);

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Créer").clicked() {
                // Validation
                if !is_valid_folder_name(&modal.folder_name) {
                    return;
                }

                if folder_exists(&modal.parent, &modal.folder_name) {
                    return;
                }

                // Création dossier
                let folder_path = modal.parent.join(&modal.folder_name);
                std::fs::create_dir_all(&folder_path).ok();

                // Création manifest
                let file_response = FastnoteFile::create_blank(
                    folder_path,
                    modal.folder_name.clone(),
                    Color32::BLUE,
                );
                match file_response {
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

                app.reload_current_project();
                app.state.modal_window = ModalWindow::None;
            }

            if ui.button("Annuler").clicked() {
                app.state.modal_window = ModalWindow::None;
            }
        });
    });
}
