use eframe::egui::{Color32, Context, Painter, Pos2, Rect};

use crate::{
    geometry::line_hits_rect,
    shapes::{arrow::Arrow, line::Line, stroke::Stroke, text::Text},
};

pub mod arrow;
pub mod line;
pub mod stroke;
pub mod text;

#[derive(Clone)]
pub enum Shape {
    Stroke(Stroke),
    Arrow(Arrow),
    Line(Line),
    Text(Text),
}

impl Shape {
    pub fn draw(&self, painter: &Painter) {
        match self {
            Shape::Stroke(stroke) => stroke.draw(painter),
            Shape::Arrow(arrow) => arrow.draw(painter),
            Shape::Line(line) => line.draw(painter),
            Shape::Text(text) => text.draw(painter),
        }
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32, context: &Context) {
        match self {
            Shape::Stroke(stroke) => stroke.update(pos, size, colour),
            Shape::Arrow(arrow) => arrow.update(pos, size, colour),
            Shape::Line(line) => line.update(pos, size, colour),
            Shape::Text(text) => text.update(pos, size, colour, context),
        }
    }

    pub fn touches(&self, thickness: f32, start: Pos2, end: Pos2) -> bool {
        match self {
            Shape::Stroke(stroke) => stroke.touches(thickness, start, end),
            Shape::Arrow(arrow) => arrow.touches(thickness, start, end),
            Shape::Line(line) => line.touches(thickness, start, end),
            Shape::Text(text) => text.touches(thickness, start, end),
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        match self {
            Shape::Stroke(stroke) => stroke.bounding_rect(),
            Shape::Arrow(arrow) => arrow.bounding_rect(),
            Shape::Line(line) => line.bounding_rect(),
            Shape::Text(text) => text.bounding_rect(),
        }
    }

    pub fn in_bounding_rect(&self, thickness: f32, start: Pos2, end: Pos2) -> bool {
        line_hits_rect(start, end, thickness, self.bounding_rect())
    }
}
