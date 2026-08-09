use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Rect};

use crate::{app::App, color_to_rgb, gpu::main_renderer::MainCallback};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuPoint {
    pos: [f32; 2],
    pressure: f32,
    color: u32,
    is_last: u32,
    _pad1: u32,
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
    let mut finished_points: Vec<GpuPoint> = vec![];
    let mut nbr_stroke = 0;
    let mut nbr_point = 0;
    for stroke in &app.state.current_file.as_ref().unwrap().strokes {
        if stroke.deleted {
            continue;
        }
        for p in &stroke.points {
            finished_points.push(GpuPoint {
                pos: [p.pos.x, p.pos.y],
                pressure: p.pressure as f32,
                color: color_to_rgb(&stroke.color),
                is_last: 0,
                _pad1: 0,
                _pad2: 0,
                _pad3: 0,
            });
            nbr_point += 1;
        }
        nbr_stroke += 1;
    }
    app.debug_info.push(format!("nbr_point {}", nbr_point));
    app.debug_info.push(format!("nbr_stroke {}", nbr_stroke));
    
    let mut current_points: Vec<GpuPoint> = vec![];
    for p in &app.state.current_file.as_ref().unwrap().current_stroke {
        current_points.push(GpuPoint {
            pos: [p.pos.x, p.pos.y],
            pressure: p.pressure as f32,
            color: color_to_rgb(&app.state.color_palette.pen.color),
            is_last: 0,
            _pad1: 0,
            _pad2: 0,
            _pad3: 0,
        });
    }

    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        MainCallback {
            current_points,
            finished_points,
            canvas_size: rect.size(),
            gpu_view: app.state.gpu_view.clone(),
        },
    ));
}
