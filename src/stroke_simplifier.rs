use eframe::egui::Pos2;

use crate::strokes::{PenStroke, StrokePoint};

pub fn simplify_stroke_rdp(stroke: &mut PenStroke, epsilon: f32) {
    if stroke.points.len() <= 4 {
        return; // déjà assez petit
    }

    let simplified = rdp_keep_endpoints(&stroke.points, epsilon);
    stroke.points = simplified;
}

pub fn rdp_keep_endpoints(points: &[StrokePoint], epsilon: f32) -> Vec<StrokePoint> {
    let n = points.len();

    // On garde les 2 premiers et les 2 derniers
    let p0 = points[0].clone();
    let p1 = points[1].clone();
    let p_last1 = points[n - 2].clone();
    let p_last = points[n - 1].clone();

    // Partie centrale à simplifier
    let middle = &points[1..(n - 1)];

    let mut simplified_middle = rdp_simplify(middle, epsilon);

    // Reconstruction
    let mut out = Vec::new();
    out.push(p0);
    out.push(p1);

    // On enlève p1 du milieu pour éviter doublon
    if simplified_middle.len() > 0 {
        simplified_middle.remove(0);
    }

    // On enlève p_last1 du milieu pour éviter doublon
    if simplified_middle.len() > 0 {
        simplified_middle.pop();
    }

    out.extend(simplified_middle);
    out.push(p_last1);
    out.push(p_last);

    out
}
pub fn rdp_simplify(points: &[StrokePoint], epsilon: f32) -> Vec<StrokePoint> {
    if points.len() < 3 {
        return points.to_vec();
    }

    let mut result = Vec::new();
    rdp_recursive(points, 0, points.len() - 1, epsilon, &mut result);

    // Ajouter le dernier point
    result.push(points[points.len() - 1].clone());
    result
}

fn rdp_recursive(
    points: &[StrokePoint],
    start: usize,
    end: usize,
    epsilon: f32,
    out: &mut Vec<StrokePoint>,
) {
    let mut max_dist = 0.0;
    let mut index = start;

    let p_start = points[start].pos;
    let p_end = points[end].pos;

    for i in (start + 1)..end {
        let dist = perpendicular_distance(points[i].pos, p_start, p_end);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }

    if max_dist > epsilon {
        rdp_recursive(points, start, index, epsilon, out);
        rdp_recursive(points, index, end, epsilon, out);
    } else {
        out.push(points[start].clone());
    }
}

fn perpendicular_distance(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ax = a.x;
    let ay = a.y;
    let bx = b.x;
    let by = b.y;
    let px = p.x;
    let py = p.y;

    let dx = bx - ax;
    let dy = by - ay;

    if dx == 0.0 && dy == 0.0 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }

    let t = ((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy);
    let t_clamped = t.clamp(0.0, 1.0);

    let proj_x = ax + t_clamped * dx;
    let proj_y = ay + t_clamped * dy;

    ((px - proj_x).powi(2) + (py - proj_y).powi(2)).sqrt()
}
