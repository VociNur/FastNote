use std::path::PathBuf;

use eframe::egui::{self, Color32};
use serde::{Deserialize, Serialize};

use crate::{
    load_persistent_data,
    pen::Pen,
    save_persistent_data,
    strokes::{PenStroke, StrokePoint},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserFile {
    pub path: PathBuf,
    pub current_stroke: Vec<StrokePoint>,
    pub strokes: Vec<PenStroke>,
    pub redraw_finished: bool,
}

impl UserFile {
    pub fn new_blank_file(path: PathBuf) -> Self {
        UserFile {
            path,
            current_stroke: vec![],
            redraw_finished: false,
            strokes: vec![],
        }
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        //path here is a file
        let json = load_persistent_data(path.clone())?;
        let mut s: Self = serde_json::from_str(&json)?;
        s.path = path;
        Ok(s)
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        // println!("Saving file: {:?}", self.path);
        save_persistent_data(self.path.clone(), &json);
        Ok(())
    }
    pub fn add_stroke_point(&mut self, stroke_point: StrokePoint) {
        self.current_stroke.push(stroke_point);
        // let err = self.save();
        // if err.is_err(){
        //     println!("Error when adding a point");
        // }//overkill
    }
    pub fn add_stroke(&mut self, stroke: PenStroke) {
        self.strokes.push(stroke);
        self.redraw_finished = true;
        let err = self.save();
        if err.is_err() {
            println!("Errorr when adding a stroke");
        }
    }
    pub fn save_current_stroke(&mut self, pen: &Pen) {
        let points = std::mem::take(&mut self.current_stroke);
        let pen_stroke = PenStroke::new(pen.color, points, pen.size);
        self.add_stroke(pen_stroke);
    }
    pub fn erase_at(&mut self, pos: egui::Pos2, radius: f32) {
        let eraser_rect = egui::Rect::from_center_size(pos, egui::vec2(radius * 2.0, radius * 2.0));

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
        }
        let err = self.save();
        if err.is_err() {
            println!("Errorr when erasing a stroke");
        }
    }
    pub fn get_cloned_strokes(&self) -> Vec<PenStroke> {
        self.strokes.clone()
    }
    pub fn get_cloned_current_stroke(&self) -> Vec<StrokePoint> {
        self.current_stroke.clone()
    }
}
