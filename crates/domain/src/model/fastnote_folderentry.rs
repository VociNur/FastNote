use crate::model::{
    fastnote_file::FastnoteFile, fastnote_folder::FastnoteFolder, fastnote_project::ItemType,
};
use crate::storage::SharedStorage;
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
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn load(storage: SharedStorage, path: PathBuf) -> anyhow::Result<Self> {
        // Charger le manifest du dossier/fichier
        let manifest_path = path.join("manifest.json");
        // let json = load_persistent_data(manifest_path)?;
        // let manifest: Manifest = serde_json::from_str(&json)?;
        let manifest = storage.load_manifest_file(&manifest_path)?;

        match manifest.r#type {
            ItemType::Folder => Ok(FolderEntry::Folder(FastnoteFolder::load(storage, path)?)),
            ItemType::File => Ok(FolderEntry::File(FastnoteFile::load(storage, path)?)),
            _ => Err(anyhow::anyhow!(
                "FolderEntry::load: type {:?} invalide pour un enfant",
                manifest.r#type
            )),
        }
    }
}
