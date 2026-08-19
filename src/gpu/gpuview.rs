use eframe::egui::{Pos2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GpuView {
    pub top_left: Pos2,
    pub zoom: f32,
}

impl Default for GpuView {
    fn default() -> Self {
        Self {
            top_left: Pos2::default(),
            zoom: 1f32,
        }
    }
}

impl GpuView {
    pub fn move_top_left(&mut self, dxy: Vec2, ppp: f32) {
        self.top_left -= dxy / self.zoom * ppp;
    }

    pub fn pinch(&mut self, new_pos: Pos2, last_pos: Pos2, other_pos: Pos2) {
        let old_dist = (last_pos - other_pos).length();
        let new_dist = (new_pos - other_pos).length();
        let scale = new_dist / old_dist;
        // Point central entre les deux doigts → centre du zoom
        let center = (new_pos + other_pos.to_vec2()) / 2.0 / self.zoom;
        // Zoom centré sur le point central
        //Le other nous sert de repère
        self.zoom *= scale;
        self.zoom = self.zoom.clamp(1.0, 20.0);
        let last_center = (last_pos + other_pos.to_vec2()) / 2.0 / self.zoom;
        self.top_left += center - last_center;
    }
}
