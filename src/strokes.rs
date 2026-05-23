use eframe::egui::Pos2;
use serde::{Deserialize, Serialize};

use eframe::egui::Color32;
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StrokePoint {
    pub pos: Pos2,
    pub pressure: f64,
    // pub tilt_x: f64, //useless ?
    // pub tilt_y: f64,
    // pub rotation: f64,
}

impl StrokePoint {
    pub fn new(pos: Pos2, pressure: f64) -> Self {
        StrokePoint { pos, pressure }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenStroke {
    pub color: Color32,
    pub points: Vec<StrokePoint>,
    pub width: f32,
}

impl Default for PenStroke {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
            points: vec![],
            width: 1f32,
        }
    }
}


impl PenStroke{
    pub fn new(color: Color32, points: Vec<StrokePoint>, width: f32)->Self{
        PenStroke{color, points, width}
    }
}
