use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Rect};

use crate::{app::App, color_to_rgb, gpu::main_renderer::MainCallback};
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuPoint {
    pos: [f32; 2],
    pressure: f32,
    color: u32,
    is_last: u32,
    deleted: u32,
    _pad2: u32,
    _pad3: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    uv: f32,
    _pad: f32,
    color: [f32; 4],
}

pub fn draw_gpu(ui: &mut egui::Ui, app: &mut App, rect: Rect) {
    let top_left_pos = app.state.gpu_view.top_left.clone();
    let Some(loaded_page) = &mut app.state.loaded_page else {
        return;
    };
    let mut finished_points: Vec<GpuPoint> = vec![];
    let mut nbr_stroke = 0;
    let mut nbr_point = 0;
    let mut nbr_point_current_stroke = 0;
    // let redraw_finished = app.state.current_file.as_ref().unwrap().redraw_finished;
    // // let redraw_finished = true;
    // app.state.current_file.as_mut().unwrap().redraw_finished = false;
    // if redraw_finished {
    for (_rx, _ry, _cx, _cy, chunk) in loaded_page.get_chunk(&top_left_pos) {
        // println!("draw r/c: {} {} {} {}", _rx, _ry, _cx, _cy);
        for stroke in &chunk.strokes {
            // if stroke.deleted {
            //     println!("stroke deleted");
            //     continue;
            // }
            for (j, p) in stroke.points.iter().enumerate() {
                finished_points.push(GpuPoint {
                    pos: [p.pos.x, p.pos.y],
                    pressure: p.pressure as f32,
                    color: color_to_rgb(&stroke.color),
                    is_last: if j == stroke.points.len() - 1 { 1 } else { 0 },
                    deleted: if stroke.deleted { 1 } else { 0 },
                    _pad2: 0,
                    _pad3: 0,
                });
                nbr_point += 1;
            }
            nbr_stroke += 1;
        }
    }

    let mut current_points: Vec<GpuPoint> = vec![];
    // for (j, p) in app
    //     .state
    //     .current_file
    //     .as_ref()
    //     .unwrap()
    //     .current_stroke
    //     .iter()
    //     .enumerate()
    for (j, p) in loaded_page.current_stroke.iter().enumerate() {
        current_points.push(GpuPoint {
            pos: [p.pos.x, p.pos.y],
            pressure: p.pressure as f32,
            color: color_to_rgb(&app.state.color_palette.pen.color),
            is_last: if j == loaded_page.current_stroke.len() - 1 {
                1
            } else {
                0
            },
            deleted: 0,
            _pad2: 0,
            _pad3: 0,
        });
        nbr_point_current_stroke += 1;
        // println!("point cur: {}", p.pos);
    }
    // if redraw_finished {
    //     app.nbr_redraw += 1;
    // }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    app.debug_info.push(format!("time {}", now));
    app.debug_info.push(format!("nbr_points {}", nbr_point));
    app.debug_info.push(format!("nbr_strokes {}", nbr_stroke));
    app.debug_info.push(format!(
        "nbr_points_current_stroke {}",
        nbr_point_current_stroke
    ));
    app.debug_info
        .push(format!("nbr redraw finished stroke {}", app.nbr_redraw));
    let mut subdivision = 10; //en général on va en prendre 10 c’est bien
    while subdivision > 2 && subdivision * nbr_point > 35_000 {
        subdivision -= 1;
    }
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        MainCallback {
            current_points,
            finished_points,
            redraw_finished: true, //redraw_finished,
            nbr_stroke,
            canvas_size: rect.size(),
            gpu_view: app.state.gpu_view.clone(),
            subdivision,
        },
    ));
}
