use eframe::egui::{self, Vec2};
use bytemuck::{Pod, Zeroable};
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
    pub vertex_count: u32,
}

impl TriangleRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("triangle"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/basic.wgsl").into()),
        });

        // Les 3 vertices du triangle (coordonnées NDC : -1 à +1)
        let vertices = vec![
            Vertex {
                position: [0.0, 0.5],
                color: [1.0, 0.0, 0.0],
            }, // haut   rouge
            Vertex {
                position: [-0.5, -0.5],
                color: [0.0, 1.0, 0.0],
            }, // bas gauche vert
            Vertex {
                position: [0.5, -0.5],
                color: [0.0, 0.0, 1.0],
            }, // bas droite bleu
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("triangle vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: None,
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

        Self {
            pipeline,
            vertex_buffer,
            vertex_count: 0,
        }
    }
}
pub struct TriangleCallback{
    pub positions: Vec<Vec2>,
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
        
        // Convertit les clics en vertices
        let mut vertices: Vec<Vertex> = vec![];
        for pos in &self.positions {
            let s = 20.0; // taille du triangle en pixels
            // converti pixels → NDC se fait dans le shader, ici on garde pixels
            vertices.push(Vertex { position: [pos.x,       pos.y - s], color: [1.0, 0.0, 0.0] });
            vertices.push(Vertex { position: [pos.x - s,   pos.y + s], color: [0.0, 1.0, 0.0] });
            vertices.push(Vertex { position: [pos.x + s,   pos.y + s], color: [0.0, 0.0, 1.0] });
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
        if renderer.vertex_count == 0 { return; }
        render_pass.set_pipeline(&renderer.pipeline);
        render_pass.set_vertex_buffer(0, renderer.vertex_buffer.slice(..));
        render_pass.draw(0..renderer.vertex_count, 0..1);
    }
}
