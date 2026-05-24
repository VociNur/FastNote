struct Uniforms {
    canvas_size: vec2<f32>,
}

struct GpuPoint {
    pos:      vec2<f32>,
    pressure: f32,
    color:    u32,
    is_last:  u32,
    _pad:     u32,
    _pad2:    u32,
    _pad3:    u32,
}

struct Vertex {
    position: vec2<f32>,
    uv:       f32,        // 0.0 = centre, 1.0 = bord
    _pad:     f32,        // alignement
    color:    vec4<f32>,
}

@group(0) @binding(0) var<uniform>             uniforms : Uniforms;
@group(0) @binding(1) var<storage, read>       points   : array<GpuPoint>;
@group(0) @binding(2) var<storage, read_write> vertices : array<Vertex>;

fn color_from_id(id: u32) -> vec4<f32> {
    if id == 1u { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let seg = gid.x;
    let n   = arrayLength(&points);
    if seg >= n - 1u { return; }

    let a = points[seg];
    if a.is_last == 1u {
        let base = seg * 6u;
        let zero = Vertex(vec2<f32>(0.0), 0.0, 0.0, vec4<f32>(0.0));
        vertices[base + 0u] = zero;
        vertices[base + 1u] = zero;
        vertices[base + 2u] = zero;
        vertices[base + 3u] = zero;
        vertices[base + 4u] = zero;
        vertices[base + 5u] = zero;
        return;
    }

    let b = points[seg + 1u];
    let dir = b.pos - a.pos;
    let len = length(dir);

    if len < 0.001 {
        let base = seg * 6u;
        let zero = Vertex(a.pos, 0.0, 0.0, vec4<f32>(0.0));
        vertices[base + 0u] = zero;
        vertices[base + 1u] = zero;
        vertices[base + 2u] = zero;
        vertices[base + 3u] = zero;
        vertices[base + 4u] = zero;
        vertices[base + 5u] = zero;
        return;
    }

    let norm = vec2<f32>(-dir.y, dir.x) / len;
    let ha = max(a.pressure * 5.0, 1.0);
    let hb = max(b.pressure * 5.0, 1.0);

    let p0 = a.pos + norm * ha;
    let p1 = a.pos - norm * ha;
    let p2 = b.pos + norm * hb;
    let p3 = b.pos - norm * hb;

    let color = color_from_id(a.color);
    let base  = seg * 6u;

    // uv = -1.0 côté gauche, +1.0 côté droit → abs() dans le fragment shader
    vertices[base + 0u] = Vertex(p0,  1.0, 0.0, color);
    vertices[base + 1u] = Vertex(p1, -1.0, 0.0, color);
    vertices[base + 2u] = Vertex(p2,  1.0, 0.0, color);
    vertices[base + 3u] = Vertex(p1, -1.0, 0.0, color);
    vertices[base + 4u] = Vertex(p3, -1.0, 0.0, color);
    vertices[base + 5u] = Vertex(p2,  1.0, 0.0, color);
}
