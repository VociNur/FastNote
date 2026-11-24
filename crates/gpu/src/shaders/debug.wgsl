struct Uniforms {
    canvas_size:  vec2<f32>,
    view_offset:  vec2<f32>,
    zoom:         f32,
    subdivisions: u32,
    _pad:         vec2<f32>,
};

struct GpuPoint {
    pos: vec2<f32>,
    pressure: f32,
    color: u32,
    is_last: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
};

struct Vertex {
    position: vec2<f32>,
    uv: f32,
    _pad: f32,
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms : Uniforms;
@group(0) @binding(1) var<storage, read> points : array<GpuPoint>;
@group(0) @binding(2) var<storage, read_write> vertices : array<Vertex>;

fn rgba_u32_to_vec4(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 24) & 0xFF),
        f32((color >> 16) & 0xFF),
        f32((color >>  8) & 0xFF),
        f32( color        & 0xFF)
    ) / 255.0;
}

@compute @workgroup_size(64)
fn cs_debug(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    let n = arrayLength(&points);

    if idx >= n {
        return;
    }

    let p = points[idx];
    let color = rgba_u32_to_vec4(p.color);

    vertices[idx] = Vertex(
        p.pos,   // position en espace monde
        0.0,     // uv = 0 → point plein
        0.0,     // pad
        color    // couleur du stroke
    );
}
