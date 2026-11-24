
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub r#type: ItemType,

    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: egui::Color32,
    #[serde(default)]
    pub is_open: bool, //osef pour un file/autre
    #[serde(default)]
    pub order: Vec<String>,
}
