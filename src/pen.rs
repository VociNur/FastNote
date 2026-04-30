use egui::Color32;
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Pen{
    pub color: Color32,
    pub size: u32,
    pub erase: bool,
}

impl Default for Pen{
    fn default() -> Self {
        Self { color: Default::default(), size: Default::default(), erase: Default::default() }
    }
}


pub const DEFAULT_PEN: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1,
    erase: false,
};


pub const DEFAULT_ERASER: Pen = Pen {
    color: Color32::from_rgb(0, 0, 0),
    size: 1,
    erase: true,
};

impl Pen{
    pub fn new(color: Color32, size: u32, erase: bool)-> Self{
        Self{
            color,
            size,
            erase
        }
    }
}
