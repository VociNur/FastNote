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
}
