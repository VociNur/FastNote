use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{load_persistent_data, projects::chunk::Chunk, save_persistent_data};

pub const CHUNKS_PER_REGION_X: usize = 4;
pub const CHUNKS_PER_REGION_Y: usize = 4;
pub const CHUNK_PIXEL_SIZE_X: f32 = 2000.0;
pub const CHUNK_PIXEL_SIZE_Y: f32 = 1000.0;
pub const REGION_PIXEL_SIZE_X: f32 = CHUNK_PIXEL_SIZE_X * CHUNKS_PER_REGION_X as f32;
pub const REGION_PIXEL_SIZE_Y: f32 = CHUNK_PIXEL_SIZE_Y * CHUNKS_PER_REGION_Y as f32;

#[derive(Debug, Clone)]
pub struct RegionCache {
    pub regions: HashMap<(i32, i32), LoadedRegion>,
    pub regions_path: PathBuf,
}

impl RegionCache {
    pub fn new(regions_path: PathBuf) -> Self {
        Self {
            regions: HashMap::new(),
            regions_path,
        }
    }

    pub fn load_region(&mut self, rx: i32, ry: i32) {
        let key = (rx, ry);

        if self.regions.contains_key(&key) {
            return; // déjà chargée
        }

        let region_path = self.regions_path.join(format!("region_{}_{}.json", rx, ry));

        let region_response = if region_path.exists() {
            LoadedRegion::load(self.regions_path.clone(), rx, ry) //pas ouf de réutiliser regions_path, mais osef
        } else {
            Ok(LoadedRegion::new(self.regions_path.clone(), rx, ry))
        };
        let Ok(region) = region_response else {
            println!(
                "Could not load: region: {:?} {} {}",
                self.regions_path, rx, ry
            );
            return;
        };
        self.regions.insert(key, region);
    }

    // pub fn save_region(&self, rx: i32, ry: i32) {
    //     println!("saving region {:?} {} {}", self.regions_path, rx, ry);
    //     if let Some(region) = self.regions.get(&(rx, ry)) {
    //         let region_path = self.regions_path.join(format!("region_{}_{}.bin", rx, ry));
    //         let response = region.save(region_path);
    //         if let Err(err) = response {
    //             println!(
    //                 "Could not save region: {:?} {} {} {:?}",
    //                 self.regions_path, rx, ry, err
    //             );
    //         }
    //     }
    // }

    pub fn save_region(&mut self, rx: i32, ry: i32) {
        let regions_path = self.regions_path.clone();
        let region = self.get_ensure_loaded_region(rx, ry);
        let res = region.save(regions_path.clone());
        if res.is_err() {
            println!("Erro r save region in region cache : {:?}", res.is_err());
        }
    }
    pub fn get_ensure_loaded_region_mut(&mut self, rx: i32, ry: i32) -> &mut LoadedRegion {
        self.ensure_region_loaded(rx, ry);
        self.regions.get_mut(&(rx, ry)).unwrap()
    }
    pub fn get_ensure_loaded_region(&mut self, rx: i32, ry: i32) -> &LoadedRegion {
        //use mut self.
        self.ensure_region_loaded(rx, ry);
        self.regions.get(&(rx, ry)).unwrap()
    }
    pub fn ensure_region_loaded(&mut self, rx: i32, ry: i32) {
        if self.get_region(rx, ry).is_none() {
            self.load_region(rx, ry);
        }
    }
    pub fn get_region_mut(&mut self, rx: i32, ry: i32) -> Option<&mut LoadedRegion> {
        self.regions.get_mut(&(rx, ry))
    }
    pub fn get_region(&self, rx: i32, ry: i32) -> Option<&LoadedRegion> {
        self.regions.get(&(rx, ry))
    }
    // pub fn get_strokes_chunk(self: &mut Self, rx: i32, ry: i32, chunk_index: u32) {
    //     let region = self.get_ensure_loaded_region(rx, ry);
    //     region.get_strokes_chunk(chunk_index)
    // }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoadedRegion {
    pub rx: i32,
    pub ry: i32,
    pub chunks: Vec<Chunk>, // 16 chunks
}

impl LoadedRegion {
    pub fn new(region_path: PathBuf, rx: i32, ry: i32) -> Self {
        let s = Self {
            rx,
            ry,
            chunks: vec![Chunk::new_blank(); 16],
        };
        let res = s.save(region_path.clone());
        if res.is_err() {
            println!(
                "Error loaded region: {:?} {} {} {:?}",
                region_path.clone(),
                rx,
                ry,
                res.err()
            );
        }
        s
    }
    pub fn filename(&self) -> String {
        format!("region_{}_{}.json", self.rx, self.ry)
    }

    pub fn filename_from_pos(rx: i32, ry: i32) -> String {
        format!("region_{}_{}.json", rx, ry)
    }
    pub fn load(regions_path: PathBuf, x: i32, y: i32) -> anyhow::Result<Self> {
        let region_path = regions_path.join(Self::filename_from_pos(x, y));
        let json = load_persistent_data(region_path)?;
        let region: Self = serde_json::from_str(&json)?;
        Ok(region)
    }

    pub fn save(&self, regions_path: PathBuf) -> anyhow::Result<()> {
        let region_path = regions_path.join(self.filename());
        let json = serde_json::to_string_pretty(&self)?;
        save_persistent_data(region_path, &json);
        Ok(())
    }
    pub fn get_chunk(self: &Self, index: usize) -> &Chunk {
        &self.chunks[index]
    }
}
