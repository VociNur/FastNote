// =========================================================
//  SHARED STRUCTS
// =========================================================

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

struct Uniforms {
    canvas_size: vec2<f32>,
    view_offset: vec2<f32>,
    zoom: f32,
    subdivisions: u32,
    _pad: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read> points: array<GpuPoint>;

@group(0) @binding(2)
var<storage, read_write> vertices: array<Vertex>;


// =========================================================
//  COMPUTE SHADER : relie les points un à un
// =========================================================

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;

    if (i + 1u >= arrayLength(&points)) {
        return;
    }

    let p0 = points[i];
    let p1 = points[i + 1u];

    let dir = normalize(p1.pos - p0.pos);
    let normal = vec2<f32>(-dir.y, dir.x);   // rotation 90°
    let half_thickness = 2.0;               // épaisseur en pixels

    let offset = normal * half_thickness;

    let v0 = p0.pos + offset;
    let v1 = p0.pos - offset;
    let v2 = p1.pos + offset;
    let v3 = p1.pos - offset;

    let base = i * 6u;

    vertices[base + 0u] = Vertex(v0, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    vertices[base + 1u] = Vertex(v1, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    vertices[base + 2u] = Vertex(v2, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    vertices[base + 3u] = Vertex(v2, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    vertices[base + 4u] = Vertex(v1, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    vertices[base + 5u] = Vertex(v3, 0.0, 0.0, vec4<f32>(0.0, 0.0, 0.0, 1.0));
}


// =========================================================
//  VERTEX SHADER : applique zoom + pan
// =========================================================

struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) uv: f32,
    @location(2) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: f32,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;

    let world_pos = (in.position - uniforms.view_offset) * uniforms.zoom;

    let ndc_x = (world_pos.x / uniforms.canvas_size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (world_pos.y / uniforms.canvas_size.y) * 2.0;

    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = in.uv;
    out.color = in.color;

    return out;
}


// =========================================================
//  FRAGMENT SHADER : simple noir
// =========================================================

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}

