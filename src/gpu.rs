use bytemuck::{Pod, Zeroable};
use eframe::egui::{self, Pos2, Vec2};
use eframe::wgpu::util::DeviceExt;
use egui_wgpu::wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

pub struct TriangleRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub vertex_count: u32,
}

impl TriangleRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/basic.wgsl").into()),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniforms"),
            contents: bytemuck::cast_slice(&[1920.0f32, 1080.0f32, 0f32, 0f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform layout"),
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

        // BindGroup — branche le buffer au layout
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Layout de la pipeline — dit quels bind groups elle utilise
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        0 => Float32x2,  // position
                        1 => Float32x3,  // color
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        //pour moment du renderer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle vertices"),
            contents: bytemuck::cast_slice(&[Vertex {
                position: [0.0, 0.0],
                color: [0.0, 0.0, 0.0],
            }]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            pipeline,
            vertex_buffer,
            bind_group,
            vertex_count: 0,
            uniform_buffer,
        }
    }
}
pub struct TriangleCallback {
    pub positions: Vec<Vec2>,
    pub canvas_size: Vec2,
}
/*
impl egui_wgpu::CallbackTrait for TriangleCallback {
    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let renderer = resources.get::<TriangleRenderer>().unwrap();
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}
*/

impl egui_wgpu::CallbackTrait for TriangleCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let renderer = resources.get_mut::<TriangleRenderer>().unwrap();

        let ppp = _screen_descriptor.pixels_per_point;
        let size = [self.canvas_size.x * ppp, self.canvas_size.y * ppp];
        
        queue.write_buffer(&renderer.uniform_buffer, 0, bytemuck::cast_slice(&size));
        // Convertit les clics en vertices
        let mut vertices: Vec<Vertex> = vec![];
        if let Some(pos) = self.positions.first() {
            // println!("pos: {:?}, canvas_size: {:?}", pos, self.canvas_size);
            let ndc_x = (pos.x / self.canvas_size.x) * 2.0 - 1.0;
            let ndc_y = 1.0 - (pos.y / self.canvas_size.y) * 2.0;
            // println!("ndc: [{}, {}]", ndc_x, ndc_y);
        }
        for pos in &self.positions {
            let s = 20.0;
            let x = pos.x * ppp;
            let y = pos.y * ppp;
            vertices.push(Vertex {
                position: [x, y - s],
                color: [1.0, 0.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x - s, y + s],
                color: [0.0, 1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [x + s, y + s],
                color: [0.0, 0.0, 1.0],
            });
        }

        renderer.vertex_count = vertices.len() as u32;
        // let n = vertices.len();
        // println!("nbr vertices: {n:?}");
        if !vertices.is_empty() {
            // Recrée le buffer avec les nouvelles données
            renderer.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("triangles"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
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
        let renderer = resources.get::<TriangleRenderer>().unwrap();
        if renderer.vertex_count == 0 {
            return;
        }
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_bind_group(0, &renderer.bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
        render_pass.draw(0..renderer.vertex_count, 0..1);
    }
}
