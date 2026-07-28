use eframe::egui::{Color32, CornerRadius, FontId, Painter, Pos2, Rect, Shape, vec2};

pub struct Text {
    pub pos: Pos2,
    pub colour: Color32,
    pub bg_colour: Color32,
    pub size: f32,
    pub text: String,
}

impl Text {
    pub fn new(text: String, pos: Pos2, bg_colour: Color32, size: f32) -> Self {
        Self {
            pos,
            colour: colour_on_bg(bg_colour),
            bg_colour,
            size,
            text,
        }
    }

    pub fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        self.pos = pos;
        self.size = size;
        if self.bg_colour != colour {
            self.bg_colour = colour;
            self.colour = colour_on_bg(colour);
        }
    }

    pub fn draw(&self, painter: &Painter) {
        let font_size = 8.0 + self.size * 1.5;
        let font_id = FontId::proportional(font_size);
        let rect_padding = 10.0;
        let rect_height = font_size + rect_padding;

        let galley =
            painter.fonts_mut(|f| f.layout_no_wrap(self.text.clone(), font_id, self.colour));
        let font_width = galley.size().x;

        painter.add(Shape::rect_filled(
            Rect::from_min_size(
                self.pos - vec2(rect_padding, 3.0),
                vec2(font_width + rect_padding * 2.0, rect_height),
            ),
            CornerRadius::same((rect_height / 5.0) as u8),
            self.bg_colour,
        ));

        painter.galley(self.pos, galley, self.colour);
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
