use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalWindow {
    None,
    Taken,
    NewProject(NewProjectModalWindow),
    NewFolder(NewFolderModalWindow),
    NewFile(NewFileModalWindow),
    Rename(RenameModalWindow),
    NewPage(NewPageModalWindow),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFileModalWindow {
    pub folder_name: String,  // nom réel du dossier
    pub display_name: String, // nom personnalisé
    // pub color: egui::Color32,
    pub parent: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewPageModalWindow {
    pub parent: PathBuf,
    pub folder_name: String,  // nom réel du dossier
    pub display_name: String, // nom personnalisé
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewProjectModalWindow {
    pub name: String,
    pub color: egui::Color32,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameModalWindow {
    pub old_path: PathBuf,
    pub new_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFolderModalWindow {
    pub parent: PathBuf,
    pub folder_name: String,
    pub display_name: String,
}
