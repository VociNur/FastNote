use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::strokes::strokes::PenStroke;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Chunk {
    pub strokes: Vec<PenStroke>,
}

impl Chunk {
    pub fn new_blank() -> Self {
        Self { strokes: vec![] }
    }

    pub fn erase_at(&mut self, pos: egui::Pos2, radius: f32) {
        let eraser_rect = egui::Rect::from_center_size(pos, egui::vec2(radius * 2.0, radius * 2.0));
        // let radius_sq = radius * radius;
        for stroke in &mut self.strokes {
            if stroke.deleted {
                continue;
            }

            // Test bbox d'abord — très rapide
            if !stroke.bbox.intersects(eraser_rect) {
                continue;
            }

            // Test précis seulement si bbox intersecte
            if stroke.intersects_point(pos, radius) {
                stroke.deleted = true;

                // println!("Deleted one");
            }
            // if stroke.touch_point(pos, radius_sq) {
            //     stroke.deleted = true;
            // }
        }
    }
}
