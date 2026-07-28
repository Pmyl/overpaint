use eframe::egui::{Color32, Painter, Pos2};

use crate::shapes::{arrow::Arrow, line::Line, stroke::Stroke, text::Text};

pub mod arrow;
pub mod line;
pub mod stroke;
pub mod text;

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

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        match self {
            Shape::Stroke(stroke) => stroke.update(pos, size, colour),
            Shape::Arrow(arrow) => arrow.update(pos, size, colour),
            Shape::Line(line) => line.update(pos, size, colour),
            Shape::Text(text) => text.update(pos, size, colour),
        }
    }
}
