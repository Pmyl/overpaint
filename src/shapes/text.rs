use std::sync::Arc;

use eframe::egui::{
    Color32, Context, CornerRadius, FontId, Galley, Painter, Pos2, Rect, Shape, vec2,
};

use crate::geometry::line_hits_rect;

#[derive(Clone)]
pub struct Text {
    pub pos: Pos2,
    pub colour: Color32,
    pub bg_colour: Color32,
    pub size: f32,
    pub text: String,

    pub font_size: f32,
    pub galley: Arc<Galley>,
    pub galley_invalidated: bool,
    pub bounding_rect: Rect,
}

const RECT_PADDING: f32 = 5.0;

fn font_size(size: f32) -> f32 {
    8.0 + size * 1.5
}

fn galley(context: &Context, font_size: f32, text: &str, colour: Color32) -> Arc<Galley> {
    context
        .fonts_mut(|f| f.layout_no_wrap(text.to_string(), FontId::proportional(font_size), colour))
}

fn rect(pos: Pos2, galley: &Galley) -> Rect {
    let text_height = galley.size().y;
    let text_width = galley.size().x;

    Rect {
        min: pos - vec2(RECT_PADDING, RECT_PADDING),
        max: Pos2::new(
            pos.x + text_width + RECT_PADDING,
            pos.y + text_height + RECT_PADDING,
        ),
    }
}

impl Text {
    pub fn new(text: String, pos: Pos2, bg_colour: Color32, size: f32, context: &Context) -> Self {
        let font_size = font_size(size);
        let colour = colour_on_bg(bg_colour);
        let galley = galley(context, font_size, &text, colour);
        let bounding_rect = rect(pos, &galley);

        Self {
            pos,
            colour,
            bg_colour,
            size,
            text,
            font_size,
            galley,
            galley_invalidated: false,
            bounding_rect,
        }
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32, context: &Context) {
        let galley_invalidated = self.galley_invalidated
            || self.bg_colour != colour
            || self.pos != pos
            || self.size != size;
        self.galley_invalidated = false;
        self.pos = pos;

        if self.bg_colour != colour {
            self.bg_colour = colour;
            self.colour = colour_on_bg(colour);
        }

        if self.size != size {
            self.size = size;
            self.font_size = font_size(self.size);
        }

        if galley_invalidated {
            self.galley = galley(context, self.font_size, &self.text, self.colour);
            self.bounding_rect = rect(pos, &self.galley);
        }
    }

    pub fn draw(&self, painter: &Painter) {
        painter.add(Shape::rect_filled(
            self.bounding_rect,
            CornerRadius::same((self.bounding_rect.height() / 5.0) as u8),
            self.bg_colour,
        ));

        painter.galley(self.pos, self.galley.clone(), self.colour);
    }

    pub fn touches(&self, thickness: f32, start: Pos2, end: Pos2) -> bool {
        line_hits_rect(start, end, thickness, self.bounding_rect())
    }

    pub fn bounding_rect(&self) -> Rect {
        self.bounding_rect
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
        self.galley_invalidated = true;
    }

    pub fn translate(&mut self, translation: eframe::egui::Vec2) {
        self.bounding_rect = self.bounding_rect.translate(translation);
        self.pos += translation;
    }
}

fn colour_on_bg(bg_colour: Color32) -> Color32 {
    let luminance =
        0.299 * bg_colour.r() as f32 + 0.587 * bg_colour.g() as f32 + 0.114 * bg_colour.b() as f32;
    if luminance > 0.5 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}
