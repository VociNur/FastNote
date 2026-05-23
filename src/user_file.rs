use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::strokes::{PenStroke, StrokePoint};
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserFile{
    
    pub path: PathBuf,
    pub current_stroke: Vec<StrokePoint>,
    pub strokes: Vec<PenStroke>,
}

impl UserFile{
    pub fn new(path: PathBuf)->Self{
        UserFile{path, current_stroke: vec![], strokes: vec![]}
    }
}
