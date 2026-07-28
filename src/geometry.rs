use eframe::egui::{Pos2, Rect, Vec2};

// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
pub fn lines_intersect(
    start_a: Pos2,
    end_a: Pos2,
    thickness_a: f32,
    start_b: Pos2,
    end_b: Pos2,
    thickness_b: f32,
) -> bool {
    // Center lines intersect.
    if segments_intersect(start_a, end_a, start_b, end_b) {
        return true;
    }

    let min_dist_sq = point_segment_distance_sq(start_a, start_b, end_b)
        .min(point_segment_distance_sq(end_a, start_b, end_b))
        .min(point_segment_distance_sq(start_b, start_a, end_a))
        .min(point_segment_distance_sq(end_b, start_a, end_a));

    let radius = (thickness_a + thickness_b) * 0.5;

    min_dist_sq <= radius * radius
}

// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
pub fn point_hits_line(point: Pos2, start: Pos2, end: Pos2, stroke_size: f32) -> bool {
    let dx = end.x - start.x;
    let dy = end.y - start.y;

    let len_sq = dx * dx + dy * dy;

    // Line is a single point
    if len_sq == 0.0 {
        let px = point.x - start.x;
        let py = point.y - start.y;

        return px * px + py * py <= (stroke_size * 0.5).powi(2);
    }

    let t = ((point.x - start.x) * dx + (point.y - start.y) * dy) / len_sq;

    let t = t.clamp(0.0, 1.0);

    let closest_x = start.x + t * dx;
    let closest_y = start.y + t * dy;

    let dist_x = point.x - closest_x;
    let dist_y = point.y - closest_y;

    let dist_sq = dist_x * dist_x + dist_y * dist_y;

    dist_sq <= (stroke_size * 0.5).powi(2)
}

pub fn line_hits_rect(start: Pos2, end: Pos2, thickness: f32, rect: Rect) -> bool {
    if rect.contains(start) || rect.contains(end) {
        return true;
    }

    let edges = [
        (
            Pos2 {
                x: rect.min.x,
                y: rect.min.y,
            },
            Pos2 {
                x: rect.max.x,
                y: rect.min.y,
            },
        ),
        (
            Pos2 {
                x: rect.max.x,
                y: rect.min.y,
            },
            Pos2 {
                x: rect.max.x,
                y: rect.max.y,
            },
        ),
        (
            Pos2 {
                x: rect.max.x,
                y: rect.max.y,
            },
            Pos2 {
                x: rect.min.x,
                y: rect.max.y,
            },
        ),
        (
            Pos2 {
                x: rect.min.x,
                y: rect.max.y,
            },
            Pos2 {
                x: rect.min.x,
                y: rect.min.y,
            },
        ),
    ];

    for (a, b) in edges {
        if lines_intersect(start, end, thickness, a, b, 0.0) {
            return true;
        }
    }

    false
}

fn point_segment_distance_sq(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;

    let len_sq = ab.dot(ab);

    if len_sq == 0.0 {
        return p.distance_sq(a);
    }

    let t = (ap.dot(ab) / len_sq).clamp(0.0, 1.0);

    let closest = Pos2 {
        x: a.x + t * ab.x,
        y: a.y + t * ab.y,
    };

    p.distance_sq(closest)
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

fn segments_intersect(start_a: Pos2, end_a: Pos2, start_b: Pos2, end_b: Pos2) -> bool {
    let r = end_a - start_a;
    let s = end_b - start_b;

    let denom = cross(r, s);

    if denom.abs() < 1e-6 {
        // Parallel (ignoring collinear overlap for simplicity)
        return false;
    }

    let qp = start_b - start_a;

    let t = cross(qp, s) / denom;
    let u = cross(qp, r) / denom;

    (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
}
