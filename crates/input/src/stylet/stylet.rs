use eframe::egui;
use input::event::tablet_tool::TabletToolType;

pub struct StyletState {
    pub pos: egui::Pos2,
    pub pressed: bool,
    pub pressure: f64, // 0.0 → 1.0
    pub distance: f64, // distance à la surface
    pub tilt_x: f64,   // inclinaison X en degrés
    pub tilt_y: f64,   // inclinaison Y en degrés
    pub rotation: f64, // rotation du stylet sur son axe
    pub slider: f64,   // molette sur le stylet (si présente)
    pub in_proximity: bool,
    pub tool_type: TabletToolType,
}

impl StyletState {
    pub fn new(
        pos: egui::Pos2,
        pressed: bool,
        pressure: f64,
        distance: f64,
        tilt_x: f64,
        tilt_y: f64,
        rotation: f64,
        slider: f64,
        in_proximity: bool,
        tool_type: TabletToolType,
    ) -> Self {
        Self {
            pos,
            pressed,
            pressure,
            distance,
            tilt_x,
            tilt_y,
            rotation,
            slider,
            in_proximity,
            tool_type,
        }
    }
}

impl Default for StyletState {
    fn default() -> Self {
        Self::new(
            egui::Pos2::default(),
            bool::default(),
            f64::default(),
            f64::default(),
            f64::default(),
            f64::default(),
            f64::default(),
            f64::default(),
            bool::default(),
            TabletToolType::Pen,
        )
    }
}
