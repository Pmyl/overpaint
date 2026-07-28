use eframe::egui::{Color32, CornerRadius, Painter, Pos2, Stroke, StrokeKind};

use crate::shapes::Shape;

pub fn draw_bounding_rect(
    painter: &Painter,
    shape: &Shape,
    previous_mouse_position: Pos2,
    mouse_position: Pos2,
) {
    let rect = shape.bounding_rect();
    let in_bounding_rect = shape.in_bounding_rect(1.0, previous_mouse_position, mouse_position);
    if in_bounding_rect {
        painter.rect(
            rect,
            CornerRadius::ZERO,
            Color32::RED.additive(),
            Stroke::new(2.0, Color32::RED),
            StrokeKind::Middle,
        );
    } else {
        painter.rect_stroke(
            rect,
            CornerRadius::ZERO,
            Stroke::new(2.0, Color32::RED),
            StrokeKind::Middle,
        );
    }
}
