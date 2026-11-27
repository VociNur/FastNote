use egui::{Color32, Pos2};

pub fn distance_point_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = (ap.dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
    let closest = a + ab * t;
    (p - closest).length()
}

pub fn distance_sq(a: Pos2, b: Pos2) -> f32 {
    a.distance_sq(b)
}

pub fn color_to_rgb(color: &Color32) -> u32 {
    ((color.r() as u32) << 24) + ((color.g() as u32) << 16) + ((color.b() as u32) << 8) + 255
}
