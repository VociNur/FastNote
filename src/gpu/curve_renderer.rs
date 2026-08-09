
// use bytemuck::{Pod, Zeroable};
// use eframe::egui::{self, Color32, Vec2};
// use eframe::wgpu::util::DeviceExt;
// use egui_wgpu::wgpu;
// use crate::strokes::{PenStroke, StrokePoint};
// use crate::gpuview::GpuView;

// #[repr(C)]
// #[derive(Copy, Clone, Pod, Zeroable)]
// struct Uniforms {
//     canvas_size:  [f32; 2],
//     view_offset:  [f32; 2],
//     zoom:         f32,
//     subdivisions: u32,
//     _pad:         [f32; 2],
// }

// pub struct CurveRenderer {
//     compute_pipeline:   wgpu::ComputePipeline,
//     render_pipeline:    wgpu::RenderPipeline,
//     points_buffer:      wgpu::Buffer,
//     vertices_buffer:    wgpu::Buffer,
//     uniform_buffer:     wgpu::Buffer,
//     compute_bind_group: wgpu::BindGroup,
//     render_bind_group:  wgpu::BindGroup,
//     pub vertex_count:   u32,
//     max_points:         usize,
// }

// impl CurveRenderer {
//     pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {

//         let current_stroke_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: Some("current stroke shader"),
//             source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/current_stroke.wgsl").into()),
//         });

//         let finished_strokes_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: Some("ink render"),
//             source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/finished_stroke.wgsl").into()),
//         });


//         let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("curve uniforms"),
//             contents: bytemuck::bytes_of(&Uniforms {
//                 canvas_size: [],
//                 view_offset: [0.0, 0.0],
//                 zoom: 1.0,
//                 subdivisions: 1,
//                 _pad: [0.0, 0.0],
//             }),
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//         });

//         let points_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("curve points"),
//             size: 10000,
//             usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("curve vertices"),
//             size: 10000,
//             usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
//             mapped_at_creation: false,
//         });

//         let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: Some("curve compute bgl"),
//             entries: &[
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Storage { read_only: true },
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 2,
//                     visibility: wgpu::ShaderStages::COMPUTE,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Storage { read_only: false },
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                     count: None,
//                 },
//             ],
//         });

//         let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: Some("curve compute bg"),
//             layout: &compute_bgl,
//             entries: &[
//                 wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
//                 wgpu::BindGroupEntry { binding: 1, resource: points_buffer.as_entire_binding() },
//                 wgpu::BindGroupEntry { binding: 2, resource: vertices_buffer.as_entire_binding() },
//             ],
//         });

//         let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: Some("curve render bgl"),
//             entries: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStages::VERTEX,
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Uniform,
//                     has_dynamic_offset: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//             }],
//         });

//         let render_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: Some("curve render bg"),
//             layout: &render_bgl,
//             entries: &[wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: uniform_buffer.as_entire_binding(),
//             }],
//         });

//         let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("curve compute layout"),
//             bind_group_layouts: &[Some(&compute_bgl)],
//             immediate_size: 0,
//         });

//         let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label: Some("curve debug compute pipeline"),
//             layout: Some(&compute_pipeline_layout),
//             module: &compute_shader,
//             entry_point: Some("cs_debug"),
//             compilation_options: Default::default(),
//             cache: None,
//         });

//         let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("curve render layout"),
//             bind_group_layouts: &[Some(&render_bgl)],
//             immediate_size: 0,
//         });

//         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("curve render pipeline"),
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &render_shader,
//                 entry_point: Some("vs_main"),
//                 buffers: &[wgpu::VertexBufferLayout {
//                     array_stride: std::mem::size_of::<Vertex>() as u64,
//                     step_mode: wgpu::VertexStepMode::Vertex,
//                     attributes: &[
//                         wgpu::VertexAttribute {
//                             format: wgpu::VertexFormat::Float32x2,
//                             offset: 0,
//                             shader_location: 0,
//                         },
//                         wgpu::VertexAttribute {
//                             format: wgpu::VertexFormat::Float32,
//                             offset: 8,
//                             shader_location: 1,
//                         },
//                         wgpu::VertexAttribute {
//                             format: wgpu::VertexFormat::Float32x4,
//                             offset: 16,
//                             shader_location: 2,
//                         },
//                     ],
//                 }],
//                 compilation_options: Default::default(),
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &render_shader,
//                 entry_point: Some("fs_main"),
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format: target_format,
//                     blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//                 compilation_options: Default::default(),
//             }),
//             primitive: wgpu::PrimitiveState::default(),
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: 4,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview_mask: None,
//             cache: None,
//         });

//         Self {
//             compute_pipeline,
//             render_pipeline,
//             points_buffer,
//             vertices_buffer,
//             uniform_buffer,
//             compute_bind_group,
//             render_bind_group,
//             vertex_count: 0,
//             max_points,
//         }
//     }
// }

// pub struct CurveCallback {
//     pub current_points: Vec<GpuPoint>,
//     pub finished_points: Vec<GpuPoint>,
//     pub canvas_size: Vec2,
//     pub gpu_view: GpuView,
// }

// impl egui_wgpu::CallbackTrait for CurveCallback {
//     fn prepare(
//         &self,
//         _device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         sd: &egui_wgpu::ScreenDescriptor,
//         encoder: &mut wgpu::CommandEncoder,
//         resources: &mut egui_wgpu::CallbackResources,
//     ) -> Vec<wgpu::CommandBuffer> {

//         let renderer = resources.get_mut::<CurveRenderer>().unwrap();
//         let ppp = sd.pixels_per_point;

//         queue.write_buffer(&renderer.uniform_buffer, 0, bytemuck::bytes_of(&Uniforms {
//             canvas_size: [self.canvas_size.x * ppp, self.canvas_size.y * ppp],
//             view_offset: [self.gpu_view.top_left.x, self.gpu_view.top_left.y],
//             zoom: self.gpu_view.zoom,
//             subdivisions: 1,
//             _pad: [0.0, 0.0],
//         }));



//         queue.write_buffer(&renderer.points_buffer, 0, bytemuck::cast_slice(&all_points));

//         let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//             label: Some("curve compute pass"),
//             timestamp_writes: None,
//         });

//         compute_pass.set_pipeline(&renderer.compute_pipeline);
//         compute_pass.set_bind_group(0, &renderer.compute_bind_group, &[]);
//         compute_pass.dispatch_workgroups((n_points + 63) / 64, 1, 1);

//         vec![]
//     }

//     fn paint(
//         &self,
//         _info: egui::PaintCallbackInfo,
//         render_pass: &mut wgpu::RenderPass<'static>,
//         resources: &egui_wgpu::CallbackResources,
//     ) {
//         let renderer = resources.get::<CurveRenderer>().unwrap();
//         if renderer.vertex_count == 0 { return; }

//         render_pass.set_pipeline(&renderer.render_pipeline);
//         render_pass.set_bind_group(0, &renderer.render_bind_group, &[]);
//         render_pass.set_vertex_buffer(0, renderer.vertices_buffer.slice(..));
//         render_pass.draw(0..renderer.vertex_count, 0..1);
//     }
// }
