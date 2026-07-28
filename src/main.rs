mod brush;
mod colour_wheel;
mod shapes;

use crate::{
    brush::Brush,
    colour_wheel::{ColourWheel, ColourWheelUi},
    shapes::{Shape, arrow::Arrow, line::Line, stroke::Stroke, text::Text},
};
use eframe::egui::{self, Color32, Pos2, ViewportBuilder};

struct SketchApp {
    shapes: Vec<Shape>,
    current_shape: Option<Shape>,
    colour_wheel: ColourWheel,
    brush: Brush,

    colour_wheel_ui: ColourWheelUi,
}

impl Default for SketchApp {
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

impl eframe::App for SketchApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Color32::from_rgba_unmultiplied(12, 12, 12, 10).to_normalized_gamma_f32()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.set_cursor_icon(egui::CursorIcon::None);

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                let painter = ui.painter();
                let screen_rect = ui.input(|i| i.content_rect());
                let ctrl = ui.input(|i| i.modifiers.ctrl);
                let alt = ui.input(|i| i.modifiers.alt);
                let scroll = ui.input(|i| {
                    i.events
                        .iter()
                        .filter_map(|e| match e {
                            egui::Event::MouseWheel { delta, .. } => Some(delta.x + delta.y),
                            _ => None,
                        })
                        .sum::<f32>()
                });
                let mouse_position = ui.input(|i| i.pointer.hover_pos()).unwrap_or(Pos2::ZERO);
                let text_events = ui.input(|i| {
                    i.events
                        .iter()
                        .filter(|e| {
                            matches!(
                                e,
                                egui::Event::Text(_)
                                    | egui::Event::Key {
                                        key: egui::Key::Backspace,
                                        pressed: true,
                                        ..
                                    }
                            )
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                });
                let has_text_events = !text_events.is_empty();

                for shape in &self.shapes {
                    shape.draw(&painter);
                }

                if let Some(current_shape) = &self.current_shape {
                    current_shape.draw(&painter);
                } else if let Some(pos) = ui.input(|i| i.pointer.latest_pos()) {
                    painter.circle(
                        pos,
                        self.brush.size / 2.0,
                        self.colour_wheel.current,
                        egui::Stroke::NONE,
                    );
                }

                painter.circle(
                    screen_rect.left_bottom(),
                    100.0,
                    self.colour_wheel.current,
                    egui::Stroke::NONE,
                );

                self.colour_wheel_ui.draw(
                    painter,
                    Pos2::new(0.0, screen_rect.height()),
                    &self.colour_wheel,
                );

                match (ctrl, scroll) {
                    (false, scroll) if scroll > 0.0 => self.colour_wheel.next(),
                    (false, scroll) if scroll < 0.0 => self.colour_wheel.prev(),
                    (true, scroll) if scroll > 0.0 => self.brush.enlarge(),
                    (true, scroll) if scroll < 0.0 => self.brush.shrink(),
                    _ => {}
                }

                if let Some(current_shape) = self.current_shape.as_mut() {
                    current_shape.update(
                        mouse_position,
                        self.brush.size,
                        self.colour_wheel.current,
                    );
                }

                let apply_text_events =
                    |text_events: &[egui::Event], mut current_text: String| -> String {
                        for text_event in text_events {
                            match text_event {
                                egui::Event::Text(text) => {
                                    current_text.push_str(&text);
                                }
                                egui::Event::Key {
                                    key: egui::Key::Backspace,
                                    pressed: true,
                                    ..
                                } => {
                                    if ctrl {
                                        current_text.clear();
                                    } else {
                                        current_text.pop();
                                    }
                                }
                                _ => (),
                            }
                        }

                        current_text
                    };

                let mouse_primary_pressed = ui.input(|i| i.pointer.primary_pressed());
                let mouse_primary_released = ui.input(|i| i.pointer.primary_released());

                match self.current_shape.as_mut() {
                    Some(Shape::Text(text_shape)) if mouse_primary_pressed => {
                        self.shapes.push(self.current_shape.take().unwrap());
                    }
                    Some(Shape::Text(text_shape)) if has_text_events => {
                        let new_text = apply_text_events(&text_events, text_shape.text.clone());

                        if !new_text.is_empty() {
                            text_shape.text = new_text;
                        } else {
                            self.current_shape.take();
                        }
                    }
                    Some(_) if mouse_primary_released => {
                        self.shapes.push(self.current_shape.take().unwrap());
                    }
                    None if mouse_primary_pressed => {
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
                    }
                    None if has_text_events => {
                        let new_text = apply_text_events(&text_events, String::new());
                        if !new_text.is_empty() {
                            self.current_shape = Some(Shape::Text(Text::new(
                                new_text,
                                mouse_position,
                                self.colour_wheel.current,
                                self.brush.size,
                            )));
                        }
                    }
                    _ => {}
                };

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ctrl && ui.input(|i| i.key_pressed(egui::Key::Z)) {
                    self.shapes.pop();
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: setup_viewport(),
        ..Default::default()
    };
    eframe::run_native(
        "overpaint",
        options,
        Box::new(|_cc| Ok(Box::new(SketchApp::default()))),
    )
}

#[cfg(target_os = "macos")]
fn setup_viewport() -> ViewportBuilder {
    use display_info::DisplayInfo;

    let display = DisplayInfo::from_point(0, 0).expect("no display found");

    let size = Vec2::new(display.width as f32, display.height as f32);
    let pos = Pos2::new(display.x as f32, display.y as f32);

    ViewportBuilder::default()
        .with_fullscreen(false)
        .with_title_shown(false)
        .with_titlebar_shown(false)
        .with_fullsize_content_view(false)
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(size)
        .with_position(pos)
        .with_always_on_top()
}

#[cfg(not(target_os = "macos"))]
fn setup_viewport() -> ViewportBuilder {
    ViewportBuilder::default()
        .with_fullscreen(true)
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top()
}
