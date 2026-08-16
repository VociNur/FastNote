use std::path::PathBuf;

use crate::{
    app::App,
    projects::{
        chunk::Chunk,
        region::{
            RegionCache, CHUNKS_PER_REGION_X, CHUNK_PIXEL_SIZE_X, CHUNK_PIXEL_SIZE_Y,
            REGION_PIXEL_SIZE_X, REGION_PIXEL_SIZE_Y,
        },
    },
    strokes::strokes::StrokePoint,
};

#[derive(Debug, Clone)]
pub struct LoadedPage {
    path: PathBuf,
    pub current_stroke: Vec<StrokePoint>,
    pub redraw_finished: bool,
    pub regions: RegionCache,
    //assets
}
impl LoadedPage {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            current_stroke: vec![],
            redraw_finished: false,
            regions: RegionCache::new(path.join("regions")),
        }
    }

    pub fn get_strokes(self: &mut Self, app: &App) -> Vec<(i32, i32, usize, usize, &Chunk)> {
        let mut out = Vec::new();

        let top_left_pos = app.state.gpu_view.top_left;

        let pixel_x = top_left_pos.x;
        let pixel_y = top_left_pos.y;
        let rx = (pixel_x / REGION_PIXEL_SIZE_X).floor() as i32;
        let ry = (pixel_y / REGION_PIXEL_SIZE_Y).floor() as i32;

        let local_x = pixel_x - (rx as f32 * REGION_PIXEL_SIZE_X);
        let local_y = pixel_y - (ry as f32 * REGION_PIXEL_SIZE_Y);

        let start_cx = (local_x / CHUNK_PIXEL_SIZE_X).floor() as usize;
        let start_cy = (local_y / CHUNK_PIXEL_SIZE_Y).floor() as usize;
        //TODO: define all regions correctly
        //
        //on y va à la bourrin
        self.regions.ensure_region_loaded(rx, ry);
        self.regions.ensure_region_loaded(rx + 1, ry);
        self.regions.ensure_region_loaded(rx, ry + 1);
        self.regions.ensure_region_loaded(rx + 1, ry + 1);
        for dcx in 0..3 {
            let cx = start_cx + dcx;
            for dcy in 0..3 {
                let cy = start_cy + dcy;

                let (downsized_cx, downsized_rx) = if cx > 3 { (cx - 4, rx + 1) } else { (cx, rx) };
                let (downsized_cy, downsized_ry) = if cy > 3 { (cy - 4, ry + 1) } else { (cy, ry) };

                let chunk_index = downsized_cy * CHUNKS_PER_REGION_X + downsized_cx;
                //Il est possible d’unwrap la prochain à l’aide de l’initialisation qui a été faite juste au dessus
                let region = self.regions.get_region(downsized_rx, downsized_ry).unwrap();
                let chunk = region.get_chunk(chunk_index);
                out.push((
                    downsized_rx,
                    downsized_ry,
                    downsized_cx,
                    downsized_cy,
                    // region.get_strokes_chunk(chunk_index as u32),
                    chunk,
                ));
            }
        }

        out
    }
}
