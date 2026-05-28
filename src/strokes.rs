use eframe::egui::{self, Pos2, Rect};
use serde::{Deserialize, Serialize};

use eframe::egui::Color32;

use crate::distance_point_to_segment;
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
    pub bbox: Rect,
    pub deleted: bool,
}

impl Default for PenStroke {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
            points: vec![],
            width: 1f32,
            bbox: Rect::ZERO,
            deleted: false,
        }
    }
}


impl PenStroke{
    pub fn new(color: Color32, points: Vec<StrokePoint>, width: f32)->Self{
        let min_x = points.iter().map(|p| p.pos.x).fold(f32::MAX, f32::min);
                let max_x = points.iter().map(|p| p.pos.x).fold(f32::MIN, f32::max);
                let min_y = points.iter().map(|p| p.pos.y).fold(f32::MAX, f32::min);
                let max_y = points.iter().map(|p| p.pos.y).fold(f32::MIN, f32::max);

                let bbox = egui::Rect::from_min_max(
                    egui::pos2(min_x, min_y),
                    egui::pos2(max_x, max_y),
                );

        
        PenStroke{color, points, width, bbox, deleted: false}
    }
}

impl PenStroke{
    
    pub fn intersects_point(self: &mut Self, pos: egui::Pos2, radius: f32) -> bool {
        for window in self.points.windows(2) {
            let a = window[0].pos;
            let b = window[1].pos;
            if distance_point_to_segment(pos, a, b) < radius {
                return true;
            }
        }
        false
    }
}
