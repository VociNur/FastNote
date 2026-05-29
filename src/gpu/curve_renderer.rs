use crate::color_to_rgb;
use crate::gpuview::GpuView;
use crate::strokes::{PenStroke, StrokePoint};
use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Color32, Vec2};
use eframe::wgpu::util::DeviceExt;
use egui_wgpu::wgpu;

// --- Structures ---

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    uv: f32,
    _pad: f32,
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuPoint {
    pos: [f32; 2],
    pressure: f32,
    color: u32,
    is_last: u32, //we can share it with r:u8, g:u8, b:u8, a:u8
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Uniforms {
    canvas_size: [f32; 2],
    view_offset: [f32; 2],
    zoom: f32,
    subdivisions: u32,
    vertex_offset: u32,
    _pad: f32,
}

// --- Renderer ---

pub struct CurveRenderer {
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,

    finished_points_buffer: wgpu::Buffer,
    current_points_buffer: wgpu::Buffer,
    vertices_buffer: wgpu::Buffer,

    uniform_buffer: wgpu::Buffer,

    compute_finished_bind_group: wgpu::BindGroup,
    compute_current_bind_group: wgpu::BindGroup,

    render_bind_group: wgpu::BindGroup,

    //pub vertex_count: u32,
    max_points: usize,
    pub finished_vertex_count: u32,
    pub current_vertex_count: u32,
    pub finished_point_count: u32,
    staging_points: Vec<GpuPoint>, // ← ajouter ici
}

impl CurveRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("curve compute"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/compute_curve.wgsl").into()),
        });
        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ink render"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/curve.wgsl").into()),
        });

        let max_points = 10_000usize;
        let max_subdivisions = 32usize;
        let max_vertices = (max_points - 1) * max_subdivisions * 6 * 2; //moit moit pour chacun
        // let max_vertices = 4_000_000usize;

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("curve uniforms"),
            contents: bytemuck::bytes_of(&Uniforms {
                canvas_size: [1920.0, 1200.0],
                view_offset: [0.0, 0.0],
                zoom: 1.0,
                subdivisions: 4,
                vertex_offset: 0,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let finished_points_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("finished point buffer"),
            size: (max_points * std::mem::size_of::<GpuPoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let current_points_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("current point buffer"),
            size: (max_points * std::mem::size_of::<GpuPoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("curve vertices"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        // --- Bind group layout compute ---
        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("curve compute bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("curve compute bg"),
        //     layout: &compute_bgl,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: uniform_buffer.as_entire_binding(),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: points_buffer.as_entire_binding(),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 2,
        //             resource: vertices_buffer.as_entire_binding(),
        //         },
        //     ],
        // });

        let compute_finished_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("finished compute bg"),
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: finished_points_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices_buffer.as_entire_binding(),
                },
            ],
        });
        let compute_current_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("current compute bg"),
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: current_points_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices_buffer.as_entire_binding(),
                },
            ],
        });

        // --- Bind group layout render ---
        let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("curve render bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("curve render bg"),
            layout: &render_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // --- Pipelines ---
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("curve compute layout"),
                bind_group_layouts: &[Some(&compute_bgl)],
                immediate_size: 0,
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("curve compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("cs_main"),
            compilation_options: Default::default(),
            cache: None,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("curve render layout"),
                bind_group_layouts: &[Some(&render_bgl)],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("curve render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32,
                            offset: 8,
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 16,
                            shader_location: 2,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            compute_pipeline,
            render_pipeline,
            finished_points_buffer,
            current_points_buffer,
            vertices_buffer,
            uniform_buffer,
            compute_finished_bind_group,
            compute_current_bind_group,
            render_bind_group,
            // vertex_count: 0,
            finished_vertex_count: 0,
            current_vertex_count: 0,
            finished_point_count: 0,
            max_points,
            staging_points: Vec::with_capacity(max_points),
        }
    }
}

// --- Callback ---

pub struct CurveCallback {
    pub current_stroke: Vec<StrokePoint>,
    pub color: Color32,
    pub strokes: Vec<PenStroke>,
    pub canvas_size: Vec2,
    pub gpu_view: GpuView,
    pub strokes_dirty: bool,
}

fn subdivisions_for_zoom(zoom: f32) -> u32 {
    // Plus on zoome, plus on subdivise
    if zoom > 4.0 {
        16
    } else if zoom > 2.0 {
        8
    } else if zoom > 1.0 {
        4
    } else {
        2
    }
}

impl egui_wgpu::CallbackTrait for CurveCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        sd: &egui_wgpu::ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources.get_mut::<CurveRenderer>().unwrap();
        let ppp = sd.pixels_per_point;
        let subs = subdivisions_for_zoom(self.gpu_view.zoom);

        queue.write_buffer(
            &renderer.uniform_buffer,
            0,
            bytemuck::bytes_of(&Uniforms {
                canvas_size: [self.canvas_size.x * ppp, self.canvas_size.y * ppp],
                view_offset: [self.gpu_view.top_left.x, self.gpu_view.top_left.y],
                zoom: self.gpu_view.zoom,
                subdivisions: subs,
                vertex_offset: 0,
                _pad: 0.0,
            }),
        );

        // ── PASS 1 : strokes terminés (seulement si dirty) ──────────────────
        if self.strokes_dirty {
            renderer.staging_points.clear();
            for stroke in &self.strokes {
                if stroke.deleted {
                    continue;
                }
                let len = stroke.points.len();
                for (i, p) in stroke.points.iter().enumerate() {
                    renderer.staging_points.push(GpuPoint {
                        pos: [p.pos.x, p.pos.y],
                        pressure: p.pressure as f32,
                        color: color_to_rgb(&stroke.color),
                        is_last: (i == len - 1) as u32,
                        _pad1: 0,
                        _pad2: 0,
                        _pad3: 0,
                    });
                }
            }
            renderer.finished_point_count = renderer.staging_points.len() as u32;
            if renderer.finished_point_count >= 2 {
                queue.write_buffer(
                    &renderer.finished_points_buffer,
                    0,
                    bytemuck::cast_slice(&renderer.staging_points),
                );
            }
        }

        let finished_n_segments = renderer.finished_point_count.saturating_sub(1);
        let finished_n_threads = finished_n_segments * subs;
        renderer.finished_vertex_count = finished_n_threads * 6;

        if finished_n_threads > 0 {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("finished compute"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&renderer.compute_pipeline);
            pass.set_bind_group(0, &renderer.compute_finished_bind_group, &[]);
            pass.dispatch_workgroups((finished_n_threads + 63) / 64, 1, 1);
        }

        // ── PASS 2 : stroke courant (toujours uploadé, toujours petit) ───────
        renderer.staging_points.clear();
        let len = self.current_stroke.len();
        for (i, p) in self.current_stroke.iter().enumerate() {
            renderer.staging_points.push(GpuPoint {
                pos: [p.pos.x, p.pos.y],
                pressure: p.pressure as f32,
                color: color_to_rgb(&self.color),
                is_last: (i == len - 1) as u32,
                _pad1: 0,
                _pad2: 0,
                _pad3: 0,
            });
        }
        let current_count = renderer.staging_points.len() as u32;
        let current_n_segments = current_count.saturating_sub(1);
        let current_n_threads = current_n_segments * subs;

        renderer.current_vertex_count = current_n_threads * 6;

        queue.write_buffer(&renderer.uniform_buffer, 0, bytemuck::bytes_of(&Uniforms {
            canvas_size:   [self.canvas_size.x * ppp, self.canvas_size.y * ppp],
            view_offset:   [self.gpu_view.top_left.x, self.gpu_view.top_left.y],
            zoom:          self.gpu_view.zoom,
            subdivisions:  subs,
            vertex_offset: finished_n_threads,  // ← écrit après les finished
            _pad:          0.,
        }));
        if current_n_threads > 0 {
            queue.write_buffer(
                &renderer.current_points_buffer,
                0,
                bytemuck::cast_slice(&renderer.staging_points),
            );
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("current compute"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&renderer.compute_pipeline);
            pass.set_bind_group(0, &renderer.compute_current_bind_group, &[]);
            pass.dispatch_workgroups((current_n_threads + 63) / 64, 1, 1);
        }

        vec![]
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let renderer = resources.get::<CurveRenderer>().unwrap();
        render_pass.set_pipeline(&renderer.render_pipeline);
        render_pass.set_bind_group(0, &renderer.render_bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.vertices_buffer.slice(..));

        let total = renderer.finished_vertex_count + renderer.current_vertex_count;
        if total > 0 {
            render_pass.draw(0..total, 0..1);  // un seul draw call suffit
        }
        
    }
}
