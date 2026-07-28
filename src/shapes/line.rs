use eframe::egui::{Color32, Painter, Pos2, Shape, Stroke};

pub struct Line {
    pub start: Pos2,
    pub end: Pos2,
    pub colour: Color32,
    pub size: f32,
}

impl Line {
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
        painter.add(Shape::circle_filled(self.end, self.size / 2.0, self.colour));
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        self.end = pos;
        self.size = size;
        self.colour = colour;
    }
}
