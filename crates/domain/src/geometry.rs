use egui::Pos2;

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
