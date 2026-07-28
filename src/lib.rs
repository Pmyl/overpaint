use eframe::egui::{
    CentralPanel, Color32, CursorIcon, Event, Frame, Key, Pos2, Rect, Stroke as EguiStroke, Ui,
    ViewportCommand, Visuals,
};

use crate::{
    brush::Brush,
    colour_wheel::{ColourWheel, ColourWheelUi},
    shapes::{Shape, arrow::Arrow, line::Line, stroke::Stroke, text::Text},
};

mod brush;
mod colour_wheel;
mod shapes;

pub struct OverpaintApp {
    shapes: Vec<Shape>,
    current_shape: Option<Shape>,
    colour_wheel: ColourWheel,
    brush: Brush,

    colour_wheel_ui: ColourWheelUi,
}

impl Default for OverpaintApp {
    fn default() -> Self {
        Self {
            shapes: Vec::new(),
            current_shape: None,
            colour_wheel: ColourWheel::default(),
            brush: Brush::default(),
            colour_wheel_ui: ColourWheelUi::default(),
        }
    }
}

impl eframe::App for OverpaintApp {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Color32::from_rgba_unmultiplied(12, 12, 12, 10).to_normalized_gamma_f32()
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        ui.set_cursor_icon(CursorIcon::None);

        CentralPanel::default().frame(Frame::NONE).show(ui, |ui| {
            let painter = ui.painter();
            let InteractionsInfo {
                ctrl,
                screen_rect,
                alt,
                escape,
                undo,
                scroll,
                mouse_position,
                text_events,
                has_text_events,
                mouse_primary_pressed,
                mouse_primary_released,
            } = extract_interactions_info(ui);

            for shape in &self.shapes {
                shape.draw(&painter);
            }

            if let Some(current_shape) = &self.current_shape {
                current_shape.draw(&painter);
            } else {
                painter.circle(
                    mouse_position,
                    self.brush.size / 2.0,
                    self.colour_wheel.current,
                    EguiStroke::NONE,
                );
            }

            self.colour_wheel_ui.draw(
                painter,
                Pos2::new(0.0, screen_rect.height()),
                &self.colour_wheel,
            );

            if scroll != 0.0 {
                match ctrl {
                    false if scroll > 0.0 => self.colour_wheel.next(),
                    false if scroll < 0.0 => self.colour_wheel.prev(),
                    true if scroll > 0.0 => self.brush.enlarge(),
                    true if scroll < 0.0 => self.brush.shrink(),
                    _ => {}
                }
            }

            if let Some(current_shape) = self.current_shape.as_mut() {
                current_shape.update(mouse_position, self.brush.size, self.colour_wheel.current);

                match current_shape {
                    Shape::Text(text_shape) if mouse_primary_pressed => {
                        self.shapes.push(self.current_shape.take().unwrap());
                    }
                    Shape::Text(text_shape) if has_text_events => {
                        let new_text =
                            apply_text_events(&text_events, text_shape.text.clone(), &ctrl);

                        if !new_text.is_empty() {
                            text_shape.text = new_text;
                        } else {
                            self.current_shape.take();
                        }
                    }
                    _ => {
                        if mouse_primary_released {
                            self.shapes.push(self.current_shape.take().unwrap())
                        }
                    }
                }
            } else {
                if mouse_primary_pressed {
                    if ctrl {
                        self.current_shape = Some(Shape::Arrow(Arrow {
                            start: mouse_position,
                            end: mouse_position,
                            colour: self.colour_wheel.current,
                            size: self.brush.size,
                        }))
                    } else if alt {
                        self.current_shape = Some(Shape::Line(Line {
                            start: mouse_position,
                            end: mouse_position,
                            colour: self.colour_wheel.current,
                            size: self.brush.size,
                        }))
                    } else {
                        self.current_shape = Some(Shape::Stroke(Stroke {
                            points: vec![mouse_position],
                            colour: self.colour_wheel.current,
                            size: self.brush.size,
                        }))
                    }
                } else if has_text_events {
                    let new_text = apply_text_events(&text_events, String::new(), &ctrl);
                    if !new_text.is_empty() {
                        self.current_shape = Some(Shape::Text(Text::new(
                            new_text,
                            mouse_position,
                            self.colour_wheel.current,
                            self.brush.size,
                        )));
                    }
                }
            }

            if escape {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }

            if undo {
                self.shapes.pop();
            }
        });
    }
}

fn extract_interactions_info(ui: &Ui) -> InteractionsInfo {
    ui.input(|i| {
        let screen_rect = i.content_rect();
        let ctrl = i.modifiers.ctrl;
        let alt = i.modifiers.alt;
        let escape = i.key_pressed(Key::Escape);
        let undo = ctrl && i.key_pressed(Key::Z);
        let scroll = i
            .events
            .iter()
            .filter_map(|e| match e {
                Event::MouseWheel { delta, .. } => Some(delta.x + delta.y),
                _ => None,
            })
            .sum::<f32>();
        let mouse_position = i.pointer.hover_pos().unwrap_or(Pos2::ZERO);
        let text_events = i
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::Text(_)
                        | Event::Key {
                            key: Key::Backspace,
                            pressed: true,
                            ..
                        }
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let has_text_events = !text_events.is_empty();
        let mouse_primary_pressed = i.pointer.primary_pressed();
        let mouse_primary_released = i.pointer.primary_released();

        InteractionsInfo {
            screen_rect,
            ctrl,
            alt,
            escape,
            undo,
            scroll,
            mouse_position,
            text_events,
            has_text_events,
            mouse_primary_pressed,
            mouse_primary_released,
        }
    })
}

fn apply_text_events(text_events: &[Event], mut current_text: String, ctrl: &bool) -> String {
    for text_event in text_events {
        match text_event {
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::Key {
                key: Key::Backspace,
                pressed: true,
                ..
            } => {
                if *ctrl {
                    current_text.clear();
                } else {
                    current_text.pop();
                }
            }
            _ => (),
        }
    }

    current_text
}

pub struct InteractionsInfo {
    screen_rect: Rect,
    ctrl: bool,
    alt: bool,
    escape: bool,
    undo: bool,
    scroll: f32,
    mouse_position: Pos2,
    text_events: Vec<Event>,
    has_text_events: bool,
    mouse_primary_pressed: bool,
    mouse_primary_released: bool,
}
