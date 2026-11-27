use std::path::PathBuf;

use anyhow::Result;
use egui::Color32;

use crate::model::fastnote_manifest::Manifest;
use crate::model::fastnote_project::ItemType;
use crate::storage::SharedStorage;
#[derive(Debug, Clone)]
pub struct FastnotePage {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub storage: SharedStorage,
}
impl FastnotePage {
    pub fn create_blank(
        storage: SharedStorage,
        page_path: PathBuf,
        name: String,
        color: Color32,
    ) -> Result<Self> {
        let manifest = Manifest {
            r#type: ItemType::Page,
            name,
            color,
            is_open: false,
            order: vec![],
        };
        let page = Self {
            path: page_path,
            manifest,
            storage: storage.clone(),
        };
        storage.save_page_file(&page)?;
        Ok(page)
    }
}
