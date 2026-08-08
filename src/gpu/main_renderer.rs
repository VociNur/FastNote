use bytemuck::{Pod, Zeroable};
use eframe::{
    egui::{self, Vec2},
    wgpu::util::DeviceExt,
};
use egui_wgpu::wgpu;

use crate::{
    gpu::main_gpu::{GpuPoint, Vertex},
    gpuview::GpuView,
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Uniforms {
    pub canvas_size: [f32; 2],
    pub view_offset: [f32; 2],
    pub zoom: f32,
    pub subdivisions: u32,
    pub _pad: [f32; 2],
}

pub struct MainRenderer {
    // --- Compute pipelines ---
    pub compute_current_pipeline: wgpu::ComputePipeline,
    pub compute_finished_pipeline: wgpu::ComputePipeline,

    // --- Render pipelines ---
    pub render_current_pipeline: wgpu::RenderPipeline,
    pub render_finished_pipeline: wgpu::RenderPipeline,

    // --- Buffers ---
    pub points_current_buffer: wgpu::Buffer,
    pub points_finished_buffer: wgpu::Buffer,

    pub vertices_current_buffer: wgpu::Buffer,
    pub vertices_finished_buffer: wgpu::Buffer,

    pub uniform_buffer: wgpu::Buffer,

    // --- Bind groups ---
    pub bg_compute_current: wgpu::BindGroup,
    pub bg_compute_finished: wgpu::BindGroup,

    pub bg_render_current: wgpu::BindGroup,
    pub bg_render_finished: wgpu::BindGroup,

    pub vertex_count_current: u32,
    pub vertex_count_finished: u32,

    pub max_points: usize,
}

impl MainRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        // --- Load shaders ---
        let cs_current = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute current"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/current_stroke.wgsl").into()),
        });

        let cs_finished = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute finished"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/finished_stroke.wgsl").into()),
        });

        let vs_current = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render current"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/current_stroke.wgsl").into()),
        });

        let vs_finished = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("render finished"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/finished_stroke.wgsl").into()),
        });

        // --- Buffers ---
        let max_points = 20_000usize;
        let max_vertices = max_points * 8;

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::bytes_of(&Uniforms {
                canvas_size: [1920.0, 1200.0],
                view_offset: [0.0, 0.0],
                zoom: 1.0,
                subdivisions: 1,
                _pad: [0.0, 0.0],
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let points_current_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("points current"),
            size: (max_points * std::mem::size_of::<GpuPoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let points_finished_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("points finished"),
            size: (max_points * std::mem::size_of::<GpuPoint>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertices_current_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertices current"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let vertices_finished_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("vertices finished"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        // --- Compute bind group layout ---
        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute bgl"),
            entries: &[
                // uniforms
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
                // input points
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
                // output vertices
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

        // --- Compute bind groups ---
        let bg_compute_current = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg compute current"),
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: points_current_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices_current_buffer.as_entire_binding(),
                },
            ],
        });

        let bg_compute_finished = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg compute finished"),
            layout: &compute_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: points_finished_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: vertices_finished_buffer.as_entire_binding(),
                },
            ],
        });

        // --- Compute pipelines ---
        let compute_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute layout"),
            bind_group_layouts: &[Some(&compute_bgl)],
            immediate_size: 0,
        });

        let compute_current_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("compute current pipeline"),
                layout: Some(&compute_layout),
                module: &cs_current,
                entry_point: Some("cs_main"),
                compilation_options: Default::default(),
                cache: None,
            });

        let compute_finished_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("compute finished pipeline"),
                layout: Some(&compute_layout),
                module: &cs_finished,
                entry_point: Some("cs_main"),
                compilation_options: Default::default(),
                cache: None,
            });

        // --- Render bind group layout ---
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

        let bg_render_current = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg render current"),
            layout: &render_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let bg_render_finished = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg render finished"),
            layout: &render_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // --- Render pipelines ---
        let render_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render layout"),
            bind_group_layouts: &[Some(&render_bgl)],
            immediate_size: 0,
        });

        let render_current_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render current pipeline"),
                layout: Some(&render_layout),
                vertex: wgpu::VertexState {
                    module: &vs_current,
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
                    module: &vs_current,
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

        let render_finished_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render finished pipeline"),
                layout: Some(&render_layout),
                vertex: wgpu::VertexState {
                    module: &vs_finished,
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
                    module: &vs_finished,
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
            compute_current_pipeline,
            compute_finished_pipeline,

            render_current_pipeline,
            render_finished_pipeline,

            points_current_buffer,
            points_finished_buffer,

            vertices_current_buffer,
            vertices_finished_buffer,

            uniform_buffer,

            bg_compute_current,
            bg_compute_finished,

            bg_render_current,
            bg_render_finished,

            vertex_count_current: 0,
            vertex_count_finished: 0,

            max_points,
        }
    }
}
impl MainRenderer {
    // ============================================================
    // 1. ÉCRITURE DES UNIFORMS
    // ============================================================

    pub fn write_uniforms(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    // ============================================================
    // 2. ÉCRITURE DES POINTS (current + finished)
    // ============================================================

    pub fn write_points_current(&self, queue: &wgpu::Queue, points: &[GpuPoint]) {
        queue.write_buffer(&self.points_current_buffer, 0, bytemuck::cast_slice(points));
    }

    pub fn write_points_finished(&self, queue: &wgpu::Queue, points: &[GpuPoint]) {
        queue.write_buffer(
            &self.points_finished_buffer,
            0,
            bytemuck::cast_slice(points),
        );
    }

    // ============================================================
    // 3. DISPATCH DES COMPUTE SHADERS
    // ============================================================

    pub fn dispatch_current_compute(&mut self, encoder: &mut wgpu::CommandEncoder, n_points: u32) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("compute current stroke"),
            timestamp_writes: None,
        });

        pass.set_pipeline(&self.compute_current_pipeline);
        pass.set_bind_group(0, &self.bg_compute_current, &[]);

        // Un segment = 2 vertices → compute shader écrit 2 * n_points
        self.vertex_count_current = n_points.saturating_sub(1) * 2;

        let workgroups = (n_points + 63) / 64;
        pass.dispatch_workgroups(workgroups, 1, 1);
    }

    pub fn dispatch_finished_compute(&mut self, encoder: &mut wgpu::CommandEncoder, n_points: u32) {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("compute finished stroke"),
            timestamp_writes: None,
        });

        pass.set_pipeline(&self.compute_finished_pipeline);
        pass.set_bind_group(0, &self.bg_compute_finished, &[]);

        // Un carré = 6 vertices
        self.vertex_count_finished = n_points * 6;

        let workgroups = (n_points + 63) / 64;
        pass.dispatch_workgroups(workgroups, 1, 1);
    }

    // ============================================================
    // 4. RENDER (current + finished)
    // ============================================================

    pub fn render_current(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.vertex_count_current == 0 {
            return;
        }

        pass.set_pipeline(&self.render_current_pipeline);
        pass.set_bind_group(0, &self.bg_render_current, &[]);
        pass.set_vertex_buffer(0, self.vertices_current_buffer.slice(..));
        pass.draw(0..self.vertex_count_current, 0..1);
    }

    pub fn render_finished(&self, pass: &mut wgpu::RenderPass<'_>) {
        if self.vertex_count_finished == 0 {
            return;
        }

        pass.set_pipeline(&self.render_finished_pipeline);
        pass.set_bind_group(0, &self.bg_render_finished, &[]);
        pass.set_vertex_buffer(0, self.vertices_finished_buffer.slice(..));
        pass.draw(0..self.vertex_count_finished, 0..1);
    }
}
pub struct MainCallback {
    pub current_points: Vec<GpuPoint>,
    pub finished_points: Vec<GpuPoint>,
    pub canvas_size: Vec2,
    pub gpu_view: GpuView,
}
impl egui_wgpu::CallbackTrait for MainCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        sd: &egui_wgpu::ScreenDescriptor,
        encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources.get_mut::<MainRenderer>().unwrap();

        // 1. Uniforms
        let ppp = sd.pixels_per_point;
        queue.write_buffer(
            &renderer.uniform_buffer,
            0,
            bytemuck::bytes_of(&Uniforms {
                canvas_size: [self.canvas_size.x * ppp, self.canvas_size.y * ppp],
                view_offset: [self.gpu_view.top_left.x, self.gpu_view.top_left.y],
                zoom: self.gpu_view.zoom,
                subdivisions: 1,
                _pad: [0.0, 0.0],
            }),
        );

        // 2. Points
        renderer.write_points_current(queue, &self.current_points);
        renderer.write_points_finished(queue, &self.finished_points);

        // 3. Compute
        renderer.dispatch_current_compute(encoder, self.current_points.len() as u32);
        renderer.dispatch_finished_compute(encoder, self.finished_points.len() as u32);

        vec![]
    }

    fn paint(
        &self,
        info: egui::PaintCallbackInfo,
        pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let renderer = resources.get::<MainRenderer>().unwrap();

        renderer.render_finished(pass);
        renderer.render_current(pass);
    }
}
