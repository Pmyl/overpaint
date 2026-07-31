use std::f32;

use eframe::egui::{self, Color32, CornerRadius, Painter, Pos2, Rect, StrokeKind};

use crate::shapes::Shape;

pub struct RectSelection {
    pub is_selecting: bool,
    pub origin: Pos2,
    pub anchor: Pos2,
    pub rect: Rect,
    pub shapes_indices: Vec<usize>,
}

impl RectSelection {
    pub fn draw(&self, painter: &Painter, shapes: &[Shape], app_counter: usize) {
        let stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
        let dash_length = 4.0;
        let gap_length = 2.0
            + if !self.is_selecting && app_counter % 100 < 50 {
                2.0
            } else {
                0.0
            };

        let top_left = self.rect.left_top();
        let top_right = self.rect.right_top();
        let bottom_right = self.rect.right_bottom();
        let bottom_left = self.rect.left_bottom();

        let top = egui::Shape::dashed_line(&[top_left, top_right], stroke, dash_length, gap_length);
        let right =
            egui::Shape::dashed_line(&[top_right, bottom_right], stroke, dash_length, gap_length);
        let bottom = egui::Shape::dashed_line(
            &[bottom_right, bottom_left],
            stroke,
            dash_length,
            gap_length,
        );
        let left =
            egui::Shape::dashed_line(&[bottom_left, top_left], stroke, dash_length, gap_length);

        painter.add(top);
        painter.add(right);
        painter.add(bottom);
        painter.add(left);

        for index in &self.shapes_indices {
            let Some(shape) = shapes.get(*index) else {
                continue;
            };

            painter.rect_stroke(
                shape.bounding_rect(),
                CornerRadius::ZERO,
                egui::Stroke::new(1.0, Color32::LIGHT_YELLOW),
                StrokeKind::Middle,
            );
        }
    }

    pub fn update(&mut self, mouse_position: Pos2, shapes: &mut [Shape]) {
        if self.is_selecting {
            self.rect = Rect::from_two_pos(self.origin, mouse_position);
            self.shapes_indices = shapes
                .iter()
                .enumerate()
                .filter(|(_, s)| s.bounding_rect().intersects(self.rect))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
        } else {
            let translation = mouse_position - self.origin;
            self.origin = mouse_position;

            self.rect = self.rect.translate(translation);
            self.shapes_indices
                .iter()
                .for_each(|&i| shapes[i].translate(translation));
        }
    }

    pub fn complete(&mut self, mouse_position: Pos2, shapes: &mut [Shape]) {
        self.update(mouse_position, shapes);

        self.anchor = mouse_position;
        self.origin = mouse_position;
        self.rect = self.shapes_indices.iter().fold(
            Rect {
                min: Pos2::new(f32::MAX, f32::MAX),
                max: Pos2::new(f32::MIN, f32::MIN),
            },
            |acc, &i| {
                let shape_rect = shapes[i].bounding_rect();
                Rect {
                    min: Pos2::new(
                        acc.min.x.min(shape_rect.min.x),
                        acc.min.y.min(shape_rect.min.y),
                    ),
                    max: Pos2::new(
                        acc.max.x.max(shape_rect.max.x),
                        acc.max.y.max(shape_rect.max.y),
                    ),
                }
            },
        );

        self.is_selecting = false;
    }

    pub fn reset_shapes(&self, shapes: &mut [Shape]) {
        let translation = self.anchor - self.origin;
        self.shapes_indices
            .iter()
            .for_each(|&i| shapes[i].translate(translation));
    }
}
