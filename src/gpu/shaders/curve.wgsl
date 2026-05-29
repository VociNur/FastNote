
struct Uniforms {
    canvas_size:  vec2<f32>,
    view_offset:  vec2<f32>,
    zoom:         f32,
    subdivisions: u32,
    vertex_offset: u32,
    _pad:         f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) uv:       f32,
    @location(2) color:    vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)       uv:            f32,
    @location(1)       color:         vec4<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;

    // Applique le zoom et le pan
    let world_pos = (in.position - uniforms.view_offset) * uniforms.zoom;

    let ndc_x =  (world_pos.x / uniforms.canvas_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / uniforms.canvas_size.y) * 2.0;

    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.uv    = in.uv;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let alpha = 1.0 - smoothstep(0.6, 1.0, abs(in.uv));
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
