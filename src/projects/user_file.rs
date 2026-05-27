use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{load_persistent_data, strokes::{PenStroke, StrokePoint}};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserFile{
    
    pub path: PathBuf,
    pub current_stroke: Vec<StrokePoint>,
    pub strokes: Vec<PenStroke>,
}

impl UserFile{
    pub fn new_blank_file(path: PathBuf) -> Self{
        UserFile{path, current_stroke: vec![], strokes: vec![]}
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {

        //path here is a file
        let json = load_persistent_data(path)?;
        let s : Self = serde_json::from_str(&json)?;
        Ok(s)
    }
}
