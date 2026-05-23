use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Vec2};
use eframe::wgpu::util::DeviceExt;
use egui_wgpu::wgpu;
use crate::get_screen_size;
use crate::strokes::{PenStroke, StrokePoint};

// --- Vertex ---
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color:    [f32; 3],
}

// --- Renderer (créé une fois au démarrage) ---
pub struct StrokeRenderer {
    pipeline:       wgpu::RenderPipeline,
    vertex_buffer:  wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group:     wgpu::BindGroup,
    pub vertex_count: u32,
}

impl StrokeRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/stroke.wgsl").into()),
        });

        let (screen_x, screen_y) = get_screen_size();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::cast_slice(&[screen_x as f32, screen_y as f32]),//0f32, 0f32
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("stroke bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("stroke bg"),
            layout:  &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("stroke pipeline layout"),
            bind_group_layouts:   &[Some(&bgl)],
            immediate_size:       0,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label:    Some("stroke vertices"),
            contents: bytemuck::cast_slice(&[Vertex { position: [0.0, 0.0], color: [0.0, 0.0, 0.0] }]),
            usage:    wgpu::BufferUsages::VERTEX,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("stroke pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module:      &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode:    wgpu::VertexStepMode::Vertex,
                    attributes:   &wgpu::vertex_attr_array![
                        0 => Float32x2,
                        1 => Float32x3,
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format:     target_format,
                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive:     wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample:   wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache:         None,
        });

        Self { pipeline, vertex_buffer, uniform_buffer, bind_group, vertex_count: 0 }
    }
}

// --- Callback ---
pub struct StrokeCallback {
    pub current_stroke: Vec<StrokePoint>,
    pub strokes:        Vec<PenStroke>,
    pub canvas_size:    Vec2,
}

fn generate_vertices(points: &[StrokePoint], color: [f32; 3], ppp: f32, out: &mut Vec<Vertex>) {
    for w in points.windows(2) {
        let a = &w[0];
        let b = &w[1];

        let ax = a.pos.x * ppp;
        let ay = a.pos.y * ppp;
        let bx = b.pos.x * ppp;
        let by = b.pos.y * ppp;

        let dx = bx - ax;
        let dy = by - ay;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 0.001 { continue; }

        let nx = -dy / len;
        let ny =  dx / len;

        let ha = (a.pressure as f32 * 5.0).max(1.0);
        let hb = (b.pressure as f32 * 5.0).max(1.0);

        let p0 = [ax + nx * ha, ay + ny * ha];
        let p1 = [ax - nx * ha, ay - ny * ha];
        let p2 = [bx + nx * hb, by + ny * hb];
        let p3 = [bx - nx * hb, by - ny * hb];

        out.push(Vertex { position: p0, color });
        out.push(Vertex { position: p1, color });
        out.push(Vertex { position: p2, color });
        out.push(Vertex { position: p1, color });
        out.push(Vertex { position: p3, color });
        out.push(Vertex { position: p2, color });
    }
}

impl egui_wgpu::CallbackTrait for StrokeCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sd: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources.get_mut::<StrokeRenderer>().unwrap();

        let ppp = sd.pixels_per_point;
        let size = [self.canvas_size.x * ppp, self.canvas_size.y * ppp];
        queue.write_buffer(&renderer.uniform_buffer, 0, bytemuck::cast_slice(&size));

        let mut vertices: Vec<Vertex> = vec![];

        // strokes terminés → rouge
        for stroke in &self.strokes {
            generate_vertices(&stroke.points, [1.0, 0.0, 0.0], ppp, &mut vertices);
        }

        // trait en cours → noir
        generate_vertices(&self.current_stroke, [0.0, 0.0, 0.0], ppp, &mut vertices);

        renderer.vertex_count = vertices.len() as u32;

        if !vertices.is_empty() {
            renderer.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label:    Some("stroke vertices"),
                contents: bytemuck::cast_slice(&vertices),
                usage:    wgpu::BufferUsages::VERTEX,
            });
        }

        vec![]
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let renderer = resources.get::<StrokeRenderer>().unwrap();
        if renderer.vertex_count == 0 { return; }
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
        render_pass.draw(0..renderer.vertex_count, 0..1);
    }
}
