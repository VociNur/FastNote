use eframe::egui::{Pos2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct GpuView {
    pub top_left: Pos2,
    zoom: f32,
    pub last_touchpad_zoom: Option<f32>,
}

impl Default for GpuView {
    fn default() -> Self {
        Self {
            top_left: Pos2::default(),
            zoom: 1f32,
            last_touchpad_zoom: None, //can be using while touchpad is zooming
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

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.zoom = self.zoom.clamp(1.0, 20.0);
    }
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    pub fn mult_zoom(&mut self, mult: f32) {
        self.set_zoom(self.get_zoom() * mult);
    }
}
