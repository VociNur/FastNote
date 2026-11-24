// Noms de fichiers/dossiers internes à la structure FastNote (voir docs/file_structuration.md)
pub const MAIN_DATA: &str = "main_data.json";
pub const NOTEBOOK: &str = "notebook";
pub const TREE_ORDER_FILE: &str = ".fastnote_order";

/// Règle métier pure : un nom de dossier/fichier valide (pas d'I/O ici).
pub fn is_valid_folder_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Interdits sur Windows + Linux
    let forbidden = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    !name.chars().any(|c| forbidden.contains(&c))
}
