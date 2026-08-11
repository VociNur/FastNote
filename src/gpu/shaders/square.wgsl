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
//  COMPUTE SHADER : génère un carré par point
// =========================================================
fn color_from_point(p: GpuPoint) -> vec4<f32> {
    // is_last vaut 0 ou 1
    let flag = f32(p.is_last);

    // Couleur rouge
    let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    // Couleur noire
    let black = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    // mix = (1-flag)*black + flag*red
    return mix(black, red, flag);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {

    let i = id.x;

    if (i >= arrayLength(&points)) {
        return;
    }

    let p = points[i];
    let center = p.pos;

    // Taille du carré (debug)
    let s = 1.0;

    // 4 coins du carré
    let p0 = center + vec2<f32>(-s, -s);
    let p1 = center + vec2<f32>( s, -s);
    let p2 = center + vec2<f32>( s,  s);
    let p3 = center + vec2<f32>(-s,  s);

    // Chaque point génère 6 vertices (2 triangles)
    let base = i * 6u;

    // let color = vec4<f32>(0.0, 0.0, 0.0, 1.0); // noir
    let color = color_from_point(p);
    vertices[base + 0u] = Vertex(p0, 0.0, 0.0, color);
    vertices[base + 1u] = Vertex(p1, 0.0, 0.0, color);
    vertices[base + 2u] = Vertex(p2, 0.0, 0.0, color);

    vertices[base + 3u] = Vertex(p0, 0.0, 0.0, color);
    vertices[base + 4u] = Vertex(p2, 0.0, 0.0, color);
    vertices[base + 5u] = Vertex(p3, 0.0, 0.0, color);
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
