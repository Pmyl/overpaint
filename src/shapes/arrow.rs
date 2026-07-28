use eframe::egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};

use crate::geometry::lines_intersect;

#[derive(Clone)]
pub struct Arrow {
    pub start: Pos2,
    pub end: Pos2,
    pub colour: Color32,
    pub size: f32,
}

impl Arrow {
    pub fn draw(&self, painter: &Painter) {
        painter.add(Shape::circle_filled(
            self.start,
            self.size / 2.0,
            self.colour,
        ));
        painter.add(Shape::line(
            vec![self.start, self.end],
            Stroke::new(self.size, self.colour),
        ));

        let dir = self.end - self.start;
        let len = dir.length();
        if len > 0.0 {
            let norm = dir / len;
            let arrow_head_len = 16.0 * self.size.max(4.0);
            let arrow_head_angle = std::f32::consts::PI / 7.0;

            let tip = self.end;
            let left = tip - norm * arrow_head_len
                + Vec2::angled(norm.angle() + arrow_head_angle) * (arrow_head_len * 0.5);
            let right = tip - norm * arrow_head_len
                + Vec2::angled(norm.angle() - arrow_head_angle) * (arrow_head_len * 0.5);

            painter.add(Shape::line(
                vec![tip, left],
                Stroke::new(self.size, self.colour),
            ));
            painter.add(Shape::line(
                vec![tip, right],
                Stroke::new(self.size, self.colour),
            ));
        }
        painter.add(Shape::circle_filled(self.end, self.size / 2.0, self.colour));
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        self.end = pos;
        self.size = size;
        self.colour = colour;
    }

    pub fn touches(&self, thickness: f32, start: Pos2, end: Pos2) -> bool {
        lines_intersect(start, end, thickness, self.start, self.end, self.size)
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect {
            min: Pos2::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y)),
            max: Pos2::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y)),
        }
    }
}
