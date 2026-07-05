use eframe::egui::{Painter, Pos2, Rect, Vec2};

use crate::ColourWheel;

#[derive(Default)]
pub struct ColourWheelUi {}

impl ColourWheelUi {
    pub fn draw(&self, painter: &Painter, pos: Pos2, colour_wheel: &ColourWheel) {
        let index = colour_wheel.index;
        let length = colour_wheel.wheel.len();
        let height = pos.y;
        let colour_size = Vec2::splat(10.0);

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
