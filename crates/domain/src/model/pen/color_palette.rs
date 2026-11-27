use serde::{Deserialize, Serialize};

use crate::{model::pen::pen::Pen, storage::SharedStorage};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorPalette {
    pub pen: Pen,
    pub palette: Vec<Pen>,
    pub is_editing: bool,
    #[serde(skip)]
    storage: Option<SharedStorage>,
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
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn save(&mut self) -> anyhow::Result<()> {
        // let json = serde_json::to_string_pretty(&self)?;
        // println!("Saving file: {:?}", self.path);
        // save_persistent_data(get_working_path().join(COLOR_PALETTE), &json);
        self.storage
            .as_ref()
            .unwrap()
            .save_color_palette_file(&self);
        Ok(())
    }
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn load(storage: SharedStorage) -> anyhow::Result<Self> {
        //path here is a file
        // let json = load_persistent_data(get_working_path().join(COLOR_PALETTE))?;
        // let s = serde_json::from_str(&json)?;
        storage.load_color_palette_file()
    }
}
