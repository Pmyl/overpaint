use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};

pub struct ColourWheel {
    pub current: Color32,
    pub index: usize,
    pub wheel: Vec<Color32>,
}

impl ColourWheel {
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.wheel.len();
        self.current = self.wheel[self.index];
    }

    pub fn prev(&mut self) {
        self.index = if self.index == 0 {
            self.wheel.len() - 1
        } else {
            self.index - 1
        };
        self.current = self.wheel[self.index];
    }
}

impl Default for ColourWheel {
    fn default() -> Self {
        let index = 0;
        let wheel = vec![
            Color32::LIGHT_BLUE,
            Color32::BLUE,
            Color32::DARK_BLUE,
            Color32::PURPLE,
            Color32::DARK_RED,
            Color32::RED,
            Color32::LIGHT_RED,
            Color32::ORANGE,
            Color32::YELLOW,
            Color32::LIGHT_GREEN,
            Color32::GREEN,
            Color32::DARK_GREEN,
            Color32::WHITE,
            Color32::LIGHT_GRAY,
            Color32::GRAY,
            Color32::DARK_GRAY,
            Color32::BLACK,
        ];
        Self {
            current: wheel[index],
            wheel,
            index,
        }
    }
}

#[derive(Default)]
pub struct ColourWheelUi {}

impl ColourWheelUi {
    pub fn draw(&self, painter: &Painter, pos: Pos2, colour_wheel: &ColourWheel) {
        let index = colour_wheel.index;
        let length = colour_wheel.wheel.len();
        let height = pos.y;
        let colour_size = Vec2::splat(10.0);

        painter.circle(pos, 100.0, colour_wheel.current, Stroke::NONE);

        for (pos_i, colour_i) in (0..length)
            .map(|offset| (index + offset) % length)
            .skip(1)
            .enumerate()
        {
            painter.rect_filled(
                Rect::from_center_size(
                    Pos2 {
                        x: pos_i as f32 * colour_size.x + colour_size.x / 2.0,
                        y: height - colour_size.y / 2.0,
                    },
                    colour_size,
                ),
                0.0,
                colour_wheel.wheel[colour_i],
            );
        }

        painter.rect_filled(
            Rect::from_center_size(
                Pos2 {
                    x: (colour_wheel.wheel.len() - 1) as f32 * colour_size.x
                        + colour_size.x / 2.0
                        + colour_size.x / 2.0,
                    y: height - colour_size.y,
                },
                colour_size * 2.0,
            ),
            0.0,
            colour_wheel.wheel[colour_wheel.index],
        );
    }
}
