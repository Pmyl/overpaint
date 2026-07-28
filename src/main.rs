mod colour_wheel_ui;

use crate::colour_wheel_ui::ColourWheelUi;
use eframe::egui::{
    self, Color32, CornerRadius, FontId, Painter, Pos2, Rect, Vec2, ViewportBuilder,
};

struct SketchApp {
    shapes: Vec<Shape>,
    current_shape: Option<Shape>,
    colour_wheel: ColourWheel,
    brush: Brush,

    colour_wheel_ui: ColourWheelUi,
}

enum Shape {
    Stroke(Stroke),
    Arrow(Arrow),
    Line(Line),
    Text(Text),
}

impl Shape {
    fn draw(&self, painter: &Painter) {
        match self {
            Shape::Stroke(stroke) => {
                if let Some(point) = stroke.points.first() {
                    painter.add(egui::Shape::circle_filled(
                        *point,
                        stroke.size / 2.0,
                        stroke.colour,
                    ));
                }
                painter.add(egui::Shape::line(
                    stroke.points.clone(),
                    egui::Stroke::new(stroke.size, stroke.colour),
                ));
                if let Some(point) = stroke.points.last() {
                    painter.add(egui::Shape::circle_filled(
                        *point,
                        stroke.size / 2.0,
                        stroke.colour,
                    ));
                }
            }
            Shape::Arrow(arrow) => {
                painter.add(egui::Shape::circle_filled(
                    arrow.start,
                    arrow.size / 2.0,
                    arrow.colour,
                ));
                painter.add(egui::Shape::line(
                    vec![arrow.start, arrow.end],
                    egui::Stroke::new(arrow.size, arrow.colour),
                ));

                let dir = arrow.end - arrow.start;
                let len = dir.length();
                if len > 0.0 {
                    let norm = dir / len;
                    let arrow_head_len = 16.0 * arrow.size.max(4.0);
                    let arrow_head_angle = std::f32::consts::PI / 7.0;

                    let tip = arrow.end;
                    let left = tip - norm * arrow_head_len
                        + egui::Vec2::angled(norm.angle() + arrow_head_angle)
                            * (arrow_head_len * 0.5);
                    let right = tip - norm * arrow_head_len
                        + egui::Vec2::angled(norm.angle() - arrow_head_angle)
                            * (arrow_head_len * 0.5);

                    painter.add(egui::Shape::line(
                        vec![tip, left],
                        egui::Stroke::new(arrow.size, arrow.colour),
                    ));
                    painter.add(egui::Shape::line(
                        vec![tip, right],
                        egui::Stroke::new(arrow.size, arrow.colour),
                    ));
                }
                painter.add(egui::Shape::circle_filled(
                    arrow.end,
                    arrow.size / 2.0,
                    arrow.colour,
                ));
            }
            Shape::Line(line) => {
                painter.add(egui::Shape::circle_filled(
                    line.start,
                    line.size / 2.0,
                    line.colour,
                ));
                painter.add(egui::Shape::line(
                    vec![line.start, line.end],
                    egui::Stroke::new(line.size, line.colour),
                ));
                painter.add(egui::Shape::circle_filled(
                    line.end,
                    line.size / 2.0,
                    line.colour,
                ));
            }
            Shape::Text(text) => {
                let font_size = 8.0 + text.size * 1.5;
                let font_id = FontId::proportional(font_size);
                let rect_padding = 10.0;
                let rect_height = font_size + rect_padding;

                let luminance = 0.299 * text.bg_colour.r() as f32
                    + 0.587 * text.bg_colour.g() as f32
                    + 0.114 * text.bg_colour.b() as f32;
                let text_colour = if luminance > 0.5 {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                };

                let galley = painter
                    .fonts_mut(|f| f.layout_no_wrap(text.text.clone(), font_id, text_colour));
                let font_width = galley.size().x;

                painter.add(egui::Shape::rect_filled(
                    Rect::from_min_size(
                        text.pos - egui::vec2(rect_padding, 3.0),
                        egui::vec2(font_width + rect_padding * 2.0, rect_height),
                    ),
                    CornerRadius::same((rect_height / 5.0) as u8),
                    text.bg_colour,
                ));

                painter.galley(text.pos, galley, text_colour);
            }
        }
    }

    fn update(&mut self, pos: Pos2, size: f32, colour: Color32) {
        match self {
            Shape::Stroke(stroke) => {
                stroke.points.push(pos);
                stroke.size = size;
                stroke.colour = colour;
            }
            Shape::Arrow(arrow) => {
                arrow.end = pos;
                arrow.size = size;
                arrow.colour = colour;
            }
            Shape::Line(line) => {
                line.end = pos;
                line.size = size;
                line.colour = colour;
            }
            Shape::Text(text) => {
                text.pos = pos;
                text.size = size;
                text.bg_colour = colour;
            }
        }
    }
}

struct Stroke {
    points: Vec<Pos2>,
    colour: Color32,
    size: f32,
}

struct Arrow {
    start: Pos2,
    end: Pos2,
    colour: Color32,
    size: f32,
}

struct Line {
    start: Pos2,
    end: Pos2,
    colour: Color32,
    size: f32,
}

struct Text {
    pos: Pos2,
    bg_colour: Color32,
    size: f32,
    text: String,
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

struct ColourWheel {
    current: Color32,
    index: usize,
    wheel: Vec<Color32>,
}

impl ColourWheel {
    fn next(&mut self) {
        self.index = (self.index + 1) % self.wheel.len();
        self.current = self.wheel[self.index];
    }

    fn prev(&mut self) {
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

struct Brush {
    size: f32,
}

impl Brush {
    fn enlarge(&mut self) {
        self.size += 1.5;
    }

    fn shrink(&mut self) {
        self.size = (self.size - 1.5).max(1.0);
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self { size: 4.0 }
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
                            if let egui::Event::Text(text) = text_event {
                                current_text.push_str(&text);
                            } else if matches!(
                                text_event,
                                egui::Event::Key {
                                    key: egui::Key::Backspace,
                                    pressed: true,
                                    ..
                                }
                            ) {
                                current_text.pop();
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
                        self.current_shape = Some(Shape::Text(Text {
                            text: new_text,
                            pos: mouse_position,
                            bg_colour: self.colour_wheel.current,
                            size: self.brush.size,
                        }));
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
