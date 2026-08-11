
// =========================================================
//  SHARED STRUCTS
// =========================================================

struct GpuPoint {
    pos: vec2<f32>,
    pressure: f32,
    color: u32,
    is_last: u32,
    _pad1: u32, //later: deleted?
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
// 
// ------------------------------------------------------------
// Catmull–Rom interpolation
// ------------------------------------------------------------
fn catmull_rom(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let t2 = t * t;
    let t3 = t2 * t;

    return 0.5 * (
        2.0 * p1 +
        (p2 - p0) * t +
        (2.0*p0 - 5.0*p1 + 4.0*p2 - p3) * t2 +
        (3.0*p1 - p0 - 3.0*p2 + p3) * t3
    );
}

fn perpendicular(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(-v.y, v.x);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {

    let i = id.x;

    if (i + 3u >= arrayLength(&points)) {
        return;
    }
    let p0 = points[i + 0u];
    let p1 = points[i + 1u];
    let p2 = points[i + 2u];
    let p3 = points[i + 3u];
    if (p0.is_last == 1u || p1.is_last == 1u || p2.is_last == 1u) {
        return;
    }

    // 10 subdivisions par segment
    let subdivisions = 10u;
    var previous_pos = p1.pos;
    for (var s = 1u; s <= subdivisions; s++) {
        let t = f32(s) / f32(subdivisions); //0 and 1 must be done, not 0 directly because it’s previous pos
        let pos = catmull_rom(p0.pos, p1.pos, p2.pos, p3.pos, t);

        // Décode couleur u32 → vec4<f32>
        let r = f32((p1.color >> 16u) & 255u) / 255.0;
        let g = f32((p1.color >> 8u) & 255u) / 255.0;
        let b = f32((p1.color >> 0u) & 255u) / 255.0;

        let base = (i * subdivisions + s-1u) * 6u;
         let dir = normalize(pos - previous_pos);
        let n = perpendicular(dir);

        let half_width = 0.25 *(p2.pressure + p1.pressure); // largeur liée à la pression, TODO moyenne de p1 et p2

        let v0 = previous_pos + n * half_width;
        let v1 = previous_pos - n * half_width;
        let v2 = pos + n * half_width;
            let v3 = pos - n * half_width;



        let color = vec4<f32>(r, g, b, 1.0);
        vertices[base+0u] = Vertex(v0, 0.0, 0.0, color);
        vertices[base+1u] = Vertex(v1, 0.0, 0.0, color);
        vertices[base+2u] = Vertex(v2, 0.0, 0.0, color);

        vertices[base+3u] = Vertex(v1, 0.0, 0.0, color);
        vertices[base+4u] = Vertex(v3, 0.0, 0.0, color);
        vertices[base+5u] = Vertex(v2, 0.0, 0.0, color);
        previous_pos = pos;
    }
    // let base = i * 6u;

    // let color = vec4<f32>(0.0, 0.0, 0.0, 1.0); // noir

    // vertices[base + 0u] = Vertex(p0, 0.0, 0.0, color);
    // vertices[base + 1u] = Vertex(p1, 0.0, 0.0, color);
    // vertices[base + 2u] = Vertex(p2, 0.0, 0.0, color);

    // vertices[base + 3u] = Vertex(p0, 0.0, 0.0, color);c’était faux ça ???
    // vertices[base + 4u] = Vertex(p2, 0.0, 0.0, color);
    // vertices[base + 5u] = Vertex(p3, 0.0, 0.0, color);
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
