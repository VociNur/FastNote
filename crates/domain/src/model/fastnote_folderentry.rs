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
