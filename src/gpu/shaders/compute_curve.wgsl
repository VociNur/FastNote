struct Uniforms {
    canvas_size: vec2<f32>,
    view_offset:  vec2<f32>,
    zoom:        f32,
    subdivisions: u32,  // nombre de subdivisions par segment selon le zoom
    vertex_offset: u32,//not used here
    _pad:        f32,
}

struct GpuPoint {
    pos:      vec2<f32>,
    pressure: f32,
    color:    u32,
    is_last:  u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
}

struct Vertex {
    position: vec2<f32>,
    uv:       f32,
    _pad:     f32,
    color:    vec4<f32>,
}

@group(0) @binding(0) var<uniform>             uniforms : Uniforms;
@group(0) @binding(1) var<storage, read>       points   : array<GpuPoint>;
@group(0) @binding(2) var<storage, read_write> vertices : array<Vertex>;

// fn color_from_id(id: u32) -> vec4<f32> {
//     if id == 1u { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }
//     return vec4<f32>(0.0, 0.0, 0.0, 1.0);
// }
//
fn rgba_u32_to_vec4(color: u32) -> vec4<f32> {
    return vec4<f32>(
        f32((color >> 24) & 0xFF),
        f32((color >> 16) & 0xFF),
        f32((color >>  8) & 0xFF),
        f32( color        & 0xFF)
    ) / 255.0;
}
// Catmull-Rom : interpole entre p1 et p2 avec p0 et p3 comme voisins
fn catmull_rom(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    return 0.5 * (
        (2.0 * p1) +
        (-p0 + p2) * t +
        (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 +
        (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3
    );
}

// Interpolation linéaire de la pression
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn emit_segment(base: u32, a_pos: vec2<f32>, b_pos: vec2<f32>, a_pressure: f32, b_pressure: f32, color: vec4<f32>) {
    let dir = b_pos - a_pos;
    let len = length(dir);
    if len < 0.001 {
        let zero = Vertex(a_pos, 0.0, 0.0, vec4<f32>(0.0));
        vertices[base + 0u] = zero;
        vertices[base + 1u] = zero;
        vertices[base + 2u] = zero;
        vertices[base + 3u] = zero;
        vertices[base + 4u] = zero;
        vertices[base + 5u] = zero;
        return;
    }

    let norm = vec2<f32>(-dir.y, dir.x) / len;

    // Épaisseur fixe en pixels écran — divisée par zoom pour rester constante
    let ha = max(a_pressure * 5.0 / uniforms.zoom, 0.5);
    let hb = max(b_pressure * 5.0 / uniforms.zoom, 0.5);

    let p0 = a_pos + norm * ha;
    let p1 = a_pos - norm * ha;
    let p2 = b_pos + norm * hb;
    let p3 = b_pos - norm * hb;

    vertices[base + 0u] = Vertex(p0,  1.0, 0.0, color);
    vertices[base + 1u] = Vertex(p1, -1.0, 0.0, color);
    vertices[base + 2u] = Vertex(p2,  1.0, 0.0, color);
    vertices[base + 3u] = Vertex(p1, -1.0, 0.0, color);
    vertices[base + 4u] = Vertex(p3, -1.0, 0.0, color);
    vertices[base + 5u] = Vertex(p2,  1.0, 0.0, color);
}

@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    // Chaque thread traite une subdivision d'un segment
    let subs = uniforms.subdivisions;
    let thread_idx = gid.x;
    
    // Quel segment et quelle subdivision ?
    let seg_idx = thread_idx / subs;
    let sub_idx = thread_idx % subs;
    
    let n = arrayLength(&points);
    if seg_idx >= n - 1u { return; }

    let a = points[seg_idx];
    if a.is_last == 1u {
        // Segment dégénéré
        let base = thread_idx * 6u;
        let zero = Vertex(a.pos, 0.0, 0.0, vec4<f32>(0.0));
        vertices[base + 0u] = zero;
        vertices[base + 1u] = zero;
        vertices[base + 2u] = zero;
        vertices[base + 3u] = zero;
        vertices[base + 4u] = zero;
        vertices[base + 5u] = zero;
        return;
    }

    let b = points[seg_idx + 1u];

    // Points voisins pour Catmull-Rom (avec clamp aux bords)
    let p1 = a.pos;
    let p2 = b.pos;

    // var p0: vec2<f32>;
    // var p3: vec2<f32>;

    // if seg_idx == 0u {
    //     // Extrapole p0 depuis p1 et p2
    //     p0 = 2.0 * points[0u].pos - points[1u].pos;
    // } else {
    //     p0 = points[seg_idx - 1u].pos;
    // }

    // if seg_idx + 2u >= n {
    //     // Extrapole p3 depuis p1 et p2
    //     p3 = 2.0 * points[n - 1u].pos - points[n - 2u].pos;
    // } else {
    //     p3 = points[seg_idx + 2u].pos;
    // }
    
    let pr1 = a.pressure;
    let pr2 = b.pressure;

    var p0: vec2<f32>;
    var p3: vec2<f32>;
    var pr0: f32;
    var pr3: f32;

    if seg_idx + 2u >= n 
    || points[seg_idx + 1u].is_last == 1u
    || points[seg_idx].is_last == 1u {
        p3  = 2.0 * points[seg_idx + 1u].pos      - points[seg_idx].pos;
        pr3 = 2.0 * points[seg_idx + 1u].pressure - points[seg_idx].pressure;
    } else {
        p3  = points[seg_idx + 2u].pos;
        pr3 = points[seg_idx + 2u].pressure;
    }

    // p0 — extrapole si le point précédent est is_last (appartient à un autre stroke)
    if seg_idx == 0u || (seg_idx > 0u && points[seg_idx - 1u].is_last == 1u) {
        p0  = 2.0 * points[seg_idx].pos      - points[seg_idx + 1u].pos;
        pr0 = 2.0 * points[seg_idx].pressure - points[seg_idx + 1u].pressure;
    } else {
        p0  = points[seg_idx - 1u].pos;
        pr0 = points[seg_idx - 1u].pressure;
    }

    // t au début et à la fin de cette subdivisions
    let t0 = f32(sub_idx)      / f32(subs);
    let t1 = f32(sub_idx + 1u) / f32(subs);

    let pos_a = catmull_rom(p0, p1, p2, p3, t0);
    let pos_b = catmull_rom(p0, p1, p2, p3, t1);

    // Pression interpolée
    let t_global_a = (f32(seg_idx) + t0) / f32(n - 1u);
    let t_global_b = (f32(seg_idx) + t1) / f32(n - 1u);
    let press_a = lerp(pr1, pr2, t0);
    let press_b = lerp(pr1, pr2, t1);

    let color = rgba_u32_to_vec4(a.color);
    let base = (uniforms.vertex_offset + thread_idx) * 6u;  
    emit_segment(base, pos_a, pos_b, press_a, press_b, color);
}
