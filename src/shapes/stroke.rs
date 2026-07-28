use eframe::{
    egui::{Color32, Painter, Pos2, Shape},
    epaint::PathStroke,
};

pub struct Stroke {
    pub points: Vec<Pos2>,
    pub colour: Color32,
    pub size: f32,
}

impl Stroke {
    pub(crate) fn draw(&self, painter: &Painter) {
        if let Some(point) = self.points.first() {
            painter.add(Shape::circle_filled(*point, self.size / 2.0, self.colour));
        }
        painter.add(Shape::line(
            self.points.clone(),
            PathStroke::new(self.size, self.colour),
        ));
        if let Some(point) = self.points.last() {
            painter.add(Shape::circle_filled(*point, self.size / 2.0, self.colour));
        }
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        self.points.push(pos);
        self.size = size;
        self.colour = colour;
    }
}
