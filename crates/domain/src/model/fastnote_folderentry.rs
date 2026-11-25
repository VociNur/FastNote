use crate::model::fastnote_manifest::Manifest;
use crate::model::{
    fastnote_file::FastnoteFile, fastnote_folder::FastnoteFolder, fastnote_project::ItemType,
};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FolderEntry {
    Folder(FastnoteFolder),
    File(FastnoteFile),
}
impl FolderEntry {
    pub fn get_path(&self) -> PathBuf {
        match self {
            FolderEntry::Folder(folder) => folder.path.clone(),
            FolderEntry::File(file) => file.path.clone(),
        }
    }
}
impl FolderEntry {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        // Charger le manifest du dossier/fichier
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        match manifest.r#type {
            ItemType::Folder => Ok(FolderEntry::Folder(FastnoteFolder::load(path)?)),
            ItemType::File => Ok(FolderEntry::File(FastnoteFile::load(path)?)),
            _ => Err(anyhow::anyhow!(
                "FolderEntry::load: type {:?} invalide pour un enfant",
                manifest.r#type
            )),
        }
    }
}
