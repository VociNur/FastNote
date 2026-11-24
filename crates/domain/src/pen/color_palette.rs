use serde::{Deserialize, Serialize};

use crate::{
    get_working_path, load_persistent_data, paths::COLOR_PALETTE, pen::pen::Pen,
    save_persistent_data,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorPalette {
    pub pen: Pen,
    pub palette: Vec<Pen>,
    pub is_editing: bool,
}

//recursive loop because of Json !
// impl Default for ColorPalette{
//     fn default() -> Self {
//         let res_self = ColorPalette::load();
//         res_self.unwrap_or(Self {
//             pen: Pen::default(),
//             palette: vec![],
//             is_editing: false,
//         })
//     }

// }

impl ColorPalette {
    pub fn add_default_pen_to_palette(&mut self) {
        self.palette.push(Pen::default());
        let _ = self.save();
    }
    pub fn save(&mut self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        // println!("Saving file: {:?}", self.path);
        save_persistent_data(get_working_path().join(COLOR_PALETTE), &json);
        Ok(())
    }
    pub fn load() -> anyhow::Result<Self> {
        //path here is a file
        let json = load_persistent_data(get_working_path().join(COLOR_PALETTE))?;
        let s = serde_json::from_str(&json)?;
        Ok(s)
    }
}
