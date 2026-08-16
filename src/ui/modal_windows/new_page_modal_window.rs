use std::{fs, path::PathBuf};

use eframe::egui::{self, Color32};

use crate::{app::App, folder_exists, is_valid_folder_name, projects::fastnote_page::FastnotePage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewPageModalWindow {
    pub parent: PathBuf,
    pub folder_name: String,  // nom réel du dossier
    pub display_name: String, // nom personnalisé
}

impl NewPageModalWindow {
    pub fn new(parent: PathBuf) -> Self {
        Self {
            parent,
            folder_name: String::new(),
            display_name: String::new(),
        }
    }
}
pub fn draw_new_page_modal_window(ui: &egui::Ui, app: &mut App, modal: &mut NewPageModalWindow) {
    egui::Modal::new(egui::Id::new("new_file_modal")).show(ui, |ui| {
        if modal.folder_name.is_empty() {
            //osef pas d’erreur ici
        } else if !is_valid_folder_name(&modal.folder_name) {
            // draw_error_banner(ui, "Caractère non valide dans le nom de fichier");
            app.push_instant_error("Caractère non valide dans le nom de fichier");
        } else if folder_exists(&modal.parent, &modal.folder_name) {
            // draw_error_banner(ui, "Un fichier portant ce nom existe déjà");
            app.push_instant_error("Un fichier portant ce nom existe déjà");
        }

        ui.heading("Nouvelle page");
        ui.add_space(10.0);

        ui.label("Nom du fichier (obligatoire) :");
        ui.text_edit_singleline(&mut modal.folder_name);

        ui.label("Nom personnalisé (facultatif) :");
        ui.text_edit_singleline(&mut modal.display_name);

        ui.add_space(10.0);

        if ui.button("Créer").clicked() {
            // Ici tu feras la vraie création plus tard
            if !is_valid_folder_name(&modal.folder_name) {
                return;
            }

            if folder_exists(&modal.parent, &modal.folder_name) {
                return;
            }
            app.state.modal_window = crate::ui::modal_windows::modal_window::ModalWindow::None;
            let create_response =
                create_fastnote_page(&modal.parent, &modal.folder_name, &modal.display_name);
            if let Err(err) = create_response {
                app.push_unsafe_minute_error(format!("Could not create page: {}", err.to_string()));
            }
            app.reload_current_project();
        }

        if ui.button("Annuler").clicked() {
            app.state.modal_window = super::modal_window::ModalWindow::None;
        }
    });
}
pub fn create_fastnote_page(
    parent: &PathBuf,
    folder_name: &str,
    display_name: &str,
) -> anyhow::Result<()> {
    let page_path = parent.join(folder_name);
    // 3. Créer le manifest.json
    let _page = FastnotePage::create_blank(
        page_path,
        display_name.to_string(),
        Color32::from_rgb(200, 200, 200),
    )?;
    Ok(())
}
