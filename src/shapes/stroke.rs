use eframe::{
    egui::{Color32, Painter, Pos2, Rect, Shape},
    epaint::PathStroke,
};

use crate::geometry::{lines_intersect, point_hits_line};

#[derive(Clone)]
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

    pub fn touches(&self, thickness: f32, start: Pos2, end: Pos2) -> bool {
        if let [point] = self.points.as_slice() {
            point_hits_line(*point, start, end, self.size)
        } else {
            self.points
                .array_windows::<2>()
                .any(|[line_start, line_end]| {
                    lines_intersect(start, end, thickness, *line_start, *line_end, self.size)
                })
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        let mut min_x: f32 = f32::MAX;
        let mut max_x: f32 = f32::MIN;
        let mut min_y: f32 = f32::MAX;
        let mut max_y: f32 = f32::MIN;

        for point in &self.points {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        Rect {
            min: Pos2::new(min_x - self.size / 2.0, min_y - self.size / 2.0),
            max: Pos2::new(max_x + self.size / 2.0, max_y + self.size / 2.0),
        }
    }

    pub fn translate(&mut self, translation: eframe::egui::Vec2) {
        for point in &mut self.points {
            *point = *point + translation;
        }
    }
}
