use crate::gpuview::GpuView;
use crate::strokes::{PenStroke, StrokePoint};
use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Vec2};
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
    color: u32,   // 0 = noir, 1 = rouge
    is_last: u32, // 1 = dernier point du stroke
    _pad: u32,
    _pad2: u32,
    _pad3: u32, // alignement 32 bytes
}

// --- Renderer ---

pub struct StrokeRenderer {
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,
    points_buffer: wgpu::Buffer,
    vertices_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    compute_bind_group: wgpu::BindGroup,
    render_bind_group: wgpu::BindGroup,
    pub vertex_count: u32,
}

impl StrokeRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/compute.wgsl").into()),
        });
        let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/stroke.wgsl").into()),
        });

        let max_points = 100_000usize;
        let max_vertices = (max_points - 1) * 6;

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::cast_slice(&[1920.0f32, 1200.0f32, 0f32, 0f32, 1f32, 0f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let points_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("points"),
            size: (max_points * std::mem::size_of::<GpuPoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertices"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        // --- Bind group layout compute ---
        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute bgl"),
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

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute bg"),
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: points_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices_buffer.as_entire_binding(),
                },
            ],
        });

        // --- Bind group layout render ---
        let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("render bgl"),
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
            label: Some("render bg"),
            layout: &render_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // --- Pipelines ---
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("compute pipeline layout"),
                bind_group_layouts: &[Some(&compute_bgl)],
                immediate_size: 0,
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("cs_main"),
            compilation_options: Default::default(),
            cache: None,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout"),
                bind_group_layouts: &[Some(&render_bgl)],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
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
                            offset: 0, // position
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32,
                            offset: 8, // uv (après position)
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 16, // color (après position + uv + _pad)
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
                count: 4, // 4x MSAA
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        Self {
            compute_pipeline,
            render_pipeline,
            points_buffer,
            vertices_buffer,
            uniform_buffer,
            compute_bind_group,
            render_bind_group,
            vertex_count: 0,
        }
    }
}

// --- Callback ---

pub struct StrokeCallback {
    pub current_stroke: Vec<StrokePoint>,
    pub strokes: Vec<PenStroke>,
    pub canvas_size: Vec2,
    pub gpu_view: GpuView,
}

impl egui_wgpu::CallbackTrait for StrokeCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        sd: &egui_wgpu::ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources.get_mut::<StrokeRenderer>().unwrap();

        let ppp = sd.pixels_per_point;
        let size = [self.canvas_size.x * ppp, self.canvas_size.y * ppp, self.gpu_view.top_left.x, self.gpu_view.top_left.y, self.gpu_view.zoom, 0f32];//pad
        queue.write_buffer(&renderer.uniform_buffer, 0, bytemuck::cast_slice(&size));

        // Construit la liste de tous les points avec métadonnées
        let mut all_points: Vec<GpuPoint> = vec![];
        // let mut total_segments = 0u32;

        for stroke in &self.strokes {
            for (i, p) in stroke.points.iter().enumerate() {
                let is_last = (i == stroke.points.len() - 1) as u32;
                // if is_last == 0 {
                //     total_segments += 1;
                // }
                all_points.push(GpuPoint {
                    pos: [p.pos.x * ppp, p.pos.y * ppp],
                    pressure: p.pressure as f32,
                    color: 1, // rouge
                    is_last,
                    _pad: 0,
                    _pad2: 0,
                    _pad3: 0,
                });
            }
        }

        for (i, p) in self.current_stroke.iter().enumerate() {
            let is_last = (i == self.current_stroke.len() - 1) as u32;
            // if is_last == 0 {
            //     total_segments += 1;
            // }
            all_points.push(GpuPoint {
                pos: [p.pos.x * ppp, p.pos.y * ppp],
                pressure: p.pressure as f32,
                color: 0, // noir
                is_last,
                _pad: 0,
                _pad2: 0,
                _pad3: 0,
            });
        }

        renderer.vertex_count = (all_points.len() as u32).saturating_sub(1) * 6;

        if all_points.len() >= 2 {
            queue.write_buffer(
                &renderer.points_buffer,
                0,
                bytemuck::cast_slice(&all_points),
            );

            let n = (all_points.len() - 1) as u32;
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&renderer.compute_pipeline);
            compute_pass.set_bind_group(0, &renderer.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups((n + 63) / 64, 1, 1);
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
        if renderer.vertex_count == 0 {
            return;
        }
        render_pass.set_pipeline(&renderer.render_pipeline);
        render_pass.set_bind_group(0, &renderer.render_bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.vertices_buffer.slice(..));
        render_pass.draw(0..renderer.vertex_count, 0..1);
    }
}
