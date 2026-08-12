use eframe::egui::Pos2;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GpuView {
    pub top_left: Pos2,
    pub zoom: f32,
}

impl Default for GpuView{
    fn default() -> Self {
        Self {top_left: Pos2::default(), zoom: 1f32}
    }
}
