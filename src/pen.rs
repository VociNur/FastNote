use eframe::egui;
use egui::Color32;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pen{
    pub color: Color32,
    pub size: f32,
}

impl Default for Pen{
    fn default() -> Self {
        Self { color: Color32::BLACK, size: Default::default() }
    }
}


pub const DEFAULT_PEN: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1.,
};


pub const DEFAULT_ERASER: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1.,
};

impl Pen{
    pub fn new(color: Color32, size: f32)-> Self{
        Self{
            color,
            size,
        }
    }
}
