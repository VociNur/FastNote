// use bytemuck::{Pod, Zeroable};
// use eframe::egui::{self, Color32, Vec2};
// use eframe::wgpu::util::DeviceExt;
// use egui_wgpu::wgpu;

// // --- Structures GPU ---

// // --- Renderer ---

// pub struct CurveRenderer {
//     pub compute_pipeline:   wgpu::ComputePipeline, // debug compute (cs_debug)
//     pub render_pipeline:    wgpu::RenderPipeline,
//     pub points_buffer:      wgpu::Buffer,
//     pub vertices_buffer:    wgpu::Buffer,
//     pub uniform_buffer:     wgpu::Buffer,
//     pub compute_bind_group: wgpu::BindGroup,
//     pub render_bind_group:  wgpu::BindGroup,
//     pub vertex_count:       u32,
//     pub max_points:         usize,
// }

// impl CurveRenderer {
//     pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
//         // --- Shaders ---
//         let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label:  Some("curve debug compute"),
//             source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/compute_debug.wgsl").into()),
//         });

//         let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label:  Some("ink render"),
//             source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/curve.wgsl").into()),
//         });

//         // --- Buffers ---
//         let max_points   = 10_000usize;
//         let max_vertices = max_points; // un vertex par point en mode debug

//         let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label:    Some("curve uniforms"),
//             contents: bytemuck::bytes_of(&Uniforms {
//                 canvas_size:  tochange,
//                 view_offset:  [0.0, 0.0],
//                 zoom:         1.0,
//                 subdivisions: 1,
//                 _pad:         [0.0, 0.0],
//             }),
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//         });

//         let points_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label:              Some("curve points"),
//             size:               (max_points * std::mem::size_of::<GpuPoint>()) as u64,
//             usage:              wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
//             label:              Some("curve vertices (debug points)"),
//             size:               (max_vertices * std::mem::size_of::<Vertex>()) as u64,
//             usage:              wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX,
//             mapped_at_creation: false,
//         });

//         // --- Bind group layout compute ---
//         let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label:   Some("curve compute bgl"),
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
//             label:   Some("curve compute bg"),
//             layout:  &compute_bgl,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: uniform_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: points_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 2,
//                     resource: vertices_buffer.as_entire_binding(),
//                 },
//             ],
//         });

//         // --- Bind group layout render ---
//         let render_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label:   Some("curve render bgl"),
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
//             label:   Some("curve render bg"),
//             layout:  &render_bgl,
//             entries: &[wgpu::BindGroupEntry {
//                 binding: 0,
//                 resource: uniform_buffer.as_entire_binding(),
//             }],
//         });

//         // --- Pipelines ---
//         let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("curve compute layout"),
//             bind_group_layouts: &[Some(&compute_bgl)],
//             immediate_size: 0,
//         });

//         let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//             label:               Some("curve debug compute pipeline"),
//             layout:              Some(&compute_pipeline_layout),
//             module:              &compute_shader,
//             entry_point:         Some("cs_debug"),
//             compilation_options: Default::default(),
//             cache:               None,
//         });

//         let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("curve render layout"),
//             bind_group_layouts: &[Some(&render_bgl)],
//             immediate_size: 0,
//         });

//         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label:  Some("curve render pipeline"),
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module:      &render_shader,
//                 entry_point: Some("vs_main"),
//                 buffers: &[wgpu::VertexBufferLayout {
//                     array_stride: std::mem::size_of::<Vertex>() as u64,
//                     step_mode:    wgpu::VertexStepMode::Vertex,
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
//                 module:      &render_shader,
//                 entry_point: Some("fs_main"),
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format:     target_format,
//                     blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//                 compilation_options: Default::default(),
//             }),
//             primitive:      wgpu::PrimitiveState::default(),
//             depth_stencil:  None,
//             multisample:    wgpu::MultisampleState {
//                 count: 4,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview_mask: None,
//             cache:          None,
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
