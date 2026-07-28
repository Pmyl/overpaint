use eframe::egui::{Color32, Painter, Pos2, Shape, Stroke, Vec2};

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
}
