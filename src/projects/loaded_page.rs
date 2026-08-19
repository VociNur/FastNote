use std::{collections::HashSet, path::PathBuf};

use eframe::egui::Pos2;

use crate::{
    projects::{
        chunk::Chunk,
        region::{
            RegionCache, CHUNKS_PER_REGION_X, CHUNKS_PER_REGION_Y, CHUNK_PIXEL_SIZE_X,
            CHUNK_PIXEL_SIZE_Y, REGION_PIXEL_SIZE_X, REGION_PIXEL_SIZE_Y,
        },
    },
    state::State,
    strokes::{
        stroke_simplifier::simplify_stroke_rdp,
        strokes::{PenStroke, StrokePoint},
    },
};

#[derive(Debug, Clone)]
pub struct LoadedPage {
    pub _path: PathBuf,
    pub current_stroke: Vec<StrokePoint>,
    pub redraw_finished: bool,
    pub regions: RegionCache,
    pub clear_buffer_finished: bool,
    pub clear_buffer_current: bool,
    //assets
}
impl LoadedPage {
    pub fn new(path: PathBuf) -> Self {
        Self {
            _path: path.clone(),
            current_stroke: vec![],
            redraw_finished: false,
            regions: RegionCache::new(path.join("regions")),
            clear_buffer_finished: true,
            clear_buffer_current: true,
        }
    }

    pub fn get_chunk(
        self: &mut Self,
        top_left_pos: &Pos2,
    ) -> Vec<(i32, i32, usize, usize, &Chunk)> {
        let mut out = Vec::new();

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
    pub fn add_stroke_point(&mut self, sp: StrokePoint) {
        self.current_stroke.push(sp);
    }
    pub fn save_current_stroke(&mut self, state: &State) {
        let pen = state.color_palette.pen.clone();
        let points = std::mem::take(&mut self.current_stroke);

        if points.is_empty() {
            return;
        }

        let mut stroke = PenStroke::new(pen.color, points, pen.size);

        simplify_stroke_rdp(&mut stroke, 0.2);

        self.distribute_stroke_into_chunks(stroke);

        self.redraw_finished = true;
        let (rx, ry) = region_of_top_left(state.gpu_view.top_left);
        self.regions.save_region(rx, ry);
        self.regions.save_region(rx + 1, ry);
        self.regions.save_region(rx, ry + 1);
        self.regions.save_region(rx + 1, ry + 1);
    }

    // ---------------------------------------------------------
    // Gomme locale au chunk
    // ---------------------------------------------------------
    pub fn erase_at(&mut self, pos: Pos2, radius: f32) {
        println!("Would like to erase");
        let regions_path = self.regions.regions_path.clone();

        // Trouver région + chunk
        let rx = (pos.x / REGION_PIXEL_SIZE_X).floor() as i32;
        let ry = (pos.y / REGION_PIXEL_SIZE_Y).floor() as i32;

        let local_x = pos.x - (rx as f32 * REGION_PIXEL_SIZE_X);
        let local_y = pos.y - (ry as f32 * REGION_PIXEL_SIZE_Y);

        let cx = (local_x / CHUNK_PIXEL_SIZE_X).floor() as usize;
        let cy = (local_y / CHUNK_PIXEL_SIZE_Y).floor() as usize;

        let chunk_index = cy * CHUNKS_PER_REGION_X + cx;

        let region = self.regions.get_ensure_loaded_region_mut(rx, ry);
        let chunk = &mut region.chunks[chunk_index];

        chunk.erase_at(pos, radius);
        let res = region.save(regions_path);
        if let Err(err) = res {
            println!("Error while saving region at erase {:?}", err);
        }
        self.redraw_finished = true;
    }
}
/*Premier plan théorique donné à copilot, p
pub fn distribute_stroke_into_chunks(&mut self, stroke: PenStroke) {
    let points = &stroke.points;
    if points.len() < 4 {
        return;
    }

    // Chunk courant et points accumulés pour ce chunk
    let mut current_chunk: Option<(i32, i32, usize)> = None;
    let mut temp_points: Vec<StrokePoint> = Vec::new();

    // Parcours par fenêtres de 4 points : ABCD, BCDE, CDEF, ...
    for window in points.windows(4) {
        let a = window[0].clone();
        let b = window[1].clone();
        let c = window[2].clone();
        let d = window[3].clone();

        // On regarde dans quels chunks va cette fenêtre
        let chunks = Self::window_chunks(&[a.clone(), b.clone(), c.clone(), d.clone()]);

        match (current_chunk, chunks.as_slice()) {
            // Premier window : on initialise
            (None, [chunk1]) => {
                current_chunk = Some(*chunk1);
                temp_points.clear();
                temp_points.extend_from_slice(&[a, b, c, d]);
            }

            // Premier window mais déjà deux chunks → on choisit le premier pour démarrer
            (None, [chunk1, _chunk2]) => {
                current_chunk = Some(*chunk1);
                temp_points.clear();
                temp_points.extend_from_slice(&[a, b, c, d]);
            }

            // Toujours dans le même chunk → on ajoute juste le nouveau point D
            (Some(chunk), [only]) if *only == chunk => {
                temp_points.push(d);
            }

            // Fenêtre dans chunk courant + un autre chunk → cas UVWX dans chunk 1 et 2
            (Some(chunk), chunks_arr) if chunks_arr.len() == 2 && chunks_arr.contains(&chunk) => {
                let other = if chunks_arr[0] == chunk { chunks_arr[1] } else { chunks_arr[0] };

                // On ajoute X (ici d) dans le vec temporaire
                temp_points.push(d.clone());

                // On sauvegarde le vec temporaire dans le chunk courant
                Self::push_temp_to_chunk(self, &stroke, chunk, &temp_points);

                // On vide le vec temporaire et on y met UVWX (ici a b c d) pour le nouveau chunk
                temp_points.clear();
                temp_points.extend_from_slice(&[a, b, c, d]);
                current_chunk = Some(*other);
            }

            // Changement de chunk simple (ABCD dans chunk1, BCDE dans chunk2)
            (Some(chunk), [new_chunk]) if *new_chunk != chunk => {
                // On sauvegarde le vec temporaire dans l’ancien chunk
                Self::push_temp_to_chunk(self, &stroke, chunk, &temp_points);

                // Nouveau chunk, nouveau vec temporaire
                temp_points.clear();
                temp_points.extend_from_slice(&[a, b, c, d]);
                current_chunk = Some(*new_chunk);
            }

            // Cas plus tordu (plus de 2 chunks) → on simplifie : on prend le premier
            (Some(chunk), chunks_arr) if !chunks_arr.is_empty() => {
                let new_chunk = chunks_arr[0];

                if new_chunk != chunk {
                    Self::push_temp_to_chunk(self, &stroke, chunk, &temp_points);
                    temp_points.clear();
                    temp_points.extend_from_slice(&[a, b, c, d]);
                    current_chunk = Some(new_chunk);
                } else {
                    temp_points.push(d);
                }
            }

            _ => {}
        }
    }

    // À la fin, on pousse le dernier vec temporaire dans le chunk courant
    if let Some(chunk) = current_chunk {
        if !temp_points.is_empty() {
            Self::push_temp_to_chunk(self, &stroke, chunk, &temp_points);
        }
    }

    self.redraw_finished = true;
}

fn window_chunks(points: &[StrokePoint; 4]) -> Vec<(i32, i32, usize)> {
    use std::collections::HashSet;
    let mut set = HashSet::new();

    for p in points {
        let px = p.pos.x;
        let py = p.pos.y;

        let rx = (px / REGION_PIXEL_SIZE_X).floor() as i32;
        let ry = (py / REGION_PIXEL_SIZE_Y).floor() as i32;

        let local_x = px - (rx as f32 * REGION_PIXEL_SIZE_X);
        let local_y = py - (ry as f32 * REGION_PIXEL_SIZE_Y);

        let cx = (local_x / CHUNK_PIXEL_SIZE_X).floor() as usize;
        let cy = (local_y / CHUNK_PIXEL_SIZE_Y).floor() as usize;

        if cx < CHUNKS_PER_REGION_X && cy < CHUNKS_PER_REGION_Y {
            let chunk_index = cy * CHUNKS_PER_REGION_X + cx;
            set.insert((rx, ry, chunk_index));
        }
    }

    set.into_iter().collect()
}

fn push_temp_to_chunk(
    &mut self,
    stroke: &PenStroke,
    chunk_id: (i32, i32, usize),
    temp_points: &Vec<StrokePoint>,
) {
    let (rx, ry, chunk_index) = chunk_id;
    let region = self.regions.get_ensure_loaded_region_mut(rx, ry);
    let chunk = &mut region.chunks[chunk_index];

    chunk.strokes.push(PenStroke {
        color: stroke.color,
        size: stroke.size,
        deleted: false,
        bbox: stroke.bbox, // tu peux recalculer si tu veux
        points: temp_points.clone(),
    });
}*/
impl LoadedPage {
    pub fn distribute_stroke_into_chunks(&mut self, stroke: PenStroke) {
        let pts = &stroke.points;
        if pts.is_empty() {
            return;
        }

        // 1. Trouver les chunks de chaque point
        let mut point_chunks: Vec<Vec<(i32, i32, usize)>> = Vec::new();

        for p in pts {
            point_chunks.push(Self::chunks_for_point(p));
        }

        // // 2. Étendre de 1 (prendre les chunks des voisins)
        // let mut extended: Vec<HashSet<(i32, i32, usize)>> = vec![HashSet::new(); pts.len()];

        // for i in 0..pts.len() {
        //     // soi-même
        //     for c in &point_chunks[i] {
        //         extended[i].insert(*c);
        //     }
        //     // voisin gauche
        //     if i > 0 {
        //         for c in &point_chunks[i - 1] {
        //             extended[i].insert(*c);
        //         }
        //     }
        //     // voisin droit
        //     if i + 1 < pts.len() {
        //         for c in &point_chunks[i + 1] {
        //             extended[i].insert(*c);
        //         }
        //     }
        // }
        // 2. Étendre de 1 en arrière et de 2 en avant
        let mut extended: Vec<HashSet<(i32, i32, usize)>> = vec![HashSet::new(); pts.len()];

        for i in 0..pts.len() {
            // soi-même
            for c in &point_chunks[i] {
                extended[i].insert(*c);
            }

            // voisin arrière (i-1)
            if i > 0 {
                for c in &point_chunks[i - 1] {
                    extended[i].insert(*c);
                }
            }

            // voisin avant (i+1)
            if i + 1 < pts.len() {
                for c in &point_chunks[i + 1] {
                    extended[i].insert(*c);
                }
            }

            // voisin avant +2 (i+2)
            if i + 2 < pts.len() {
                for c in &point_chunks[i + 2] {
                    extended[i].insert(*c);
                }
            }
        }

        // 3. Pour chaque chunk, on extrait les points consécutifs
        use std::collections::HashMap;
        let mut per_chunk: HashMap<(i32, i32, usize), Vec<Vec<StrokePoint>>> = HashMap::new();

        for (i, chunkset) in extended.iter().enumerate() {
            for chunk in chunkset {
                per_chunk.entry(*chunk).or_default();

                let groups = per_chunk.get_mut(chunk).unwrap();

                if groups.is_empty() {
                    // première séquence
                    groups.push(vec![pts[i].clone()]);
                } else {
                    let last_group = groups.last_mut().unwrap();

                    // si le point précédent contenait aussi ce chunk → on continue la séquence
                    if i > 0 && extended[i - 1].contains(chunk) {
                        last_group.push(pts[i].clone());
                    } else {
                        // sinon nouvelle séquence
                        groups.push(vec![pts[i].clone()]);
                    }
                }
            }
        }

        // 4. On pousse les séquences dans les chunks
        for (chunk_id, groups) in per_chunk {
            let (rx, ry, chunk_index) = chunk_id;
            let region = self.regions.get_ensure_loaded_region_mut(rx, ry);
            let chunk = &mut region.chunks[chunk_index];

            for seq in groups {
                if seq.len() >= 2 {
                    chunk.strokes.push(PenStroke {
                        color: stroke.color,
                        width: stroke.width,
                        deleted: false,
                        bbox: stroke.bbox,
                        points: seq,
                    });
                }
            }
        }

        self.redraw_finished = true;
        //on sauvegarde les régions (4 régions c’est okay pour l’instant, un peu bourrin)
        //
    }
    fn chunks_for_point(p: &StrokePoint) -> Vec<(i32, i32, usize)> {
        let px = p.pos.x;
        let py = p.pos.y;

        let rx = (px / REGION_PIXEL_SIZE_X).floor() as i32;
        let ry = (py / REGION_PIXEL_SIZE_Y).floor() as i32;

        let local_x = px - (rx as f32 * REGION_PIXEL_SIZE_X);
        let local_y = py - (ry as f32 * REGION_PIXEL_SIZE_Y);

        let cx = (local_x / CHUNK_PIXEL_SIZE_X).floor() as usize;
        let cy = (local_y / CHUNK_PIXEL_SIZE_Y).floor() as usize;

        if cx < CHUNKS_PER_REGION_X && cy < CHUNKS_PER_REGION_Y {
            let chunk_index = cy * CHUNKS_PER_REGION_X + cx;
            vec![(rx, ry, chunk_index)]
        } else {
            vec![]
        }
    }
}
pub fn region_of_top_left(top_left_pos: Pos2) -> (i32, i32) {
    let x = top_left_pos.x;
    let y = top_left_pos.y;

    let rx = (x / REGION_PIXEL_SIZE_X).floor() as i32;
    let ry = (y / REGION_PIXEL_SIZE_Y).floor() as i32;

    (rx, ry)
}
