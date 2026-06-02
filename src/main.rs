use eframe::egui::{self, Color32, Painter, Pos2, Rect, ViewportBuilder};

struct SketchApp {
    shapes: Vec<Shape>,
    current_shape: Option<Shape>,
    colour_wheel: ColourWheel,
    brush: Brush,
}

enum Shape {
    Stroke(Stroke),
    Arrow(Arrow),
    Line(Line),
}

impl Shape {
    fn draw(&self, painter: &Painter) {
        match self {
            Shape::Stroke(stroke) => {
                painter.add(egui::Shape::line(
                    stroke.points.clone(),
                    egui::Stroke::new(stroke.size, stroke.colour),
                ));
            }
            Shape::Arrow(arrow) => {
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
            }
            Shape::Line(line) => {
                painter.add(egui::Shape::line(
                    vec![line.start, line.end],
                    egui::Stroke::new(line.size, line.colour),
                ));
            }
        }
    }

    fn update_pos(&mut self, pos: Pos2) {
        match self {
            Shape::Stroke(stroke) => stroke.points.push(pos),
            Shape::Arrow(arrow) => arrow.end = pos,
            Shape::Line(line) => line.end = pos,
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

impl Default for SketchApp {
    fn default() -> Self {
        Self {
            shapes: Vec::new(),
            current_shape: None,
            colour_wheel: ColourWheel::default(),
            brush: Brush::default(),
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
        self.size += 1.0;
    }

    fn shrink(&mut self) {
        self.size = (self.size - 1.0).max(1.0);
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

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let painter = ui.painter();
                let screen_rect = ctx.input(|i| i.screen_rect);
                let ctrl = ui.input(|i| i.modifiers.ctrl);
                let alt = ui.input(|i| i.modifiers.alt);
                let scroll = ui.input(|i| i.raw_scroll_delta.x + i.raw_scroll_delta.y);

                painter.rect(
                    Rect::from_two_pos(
                        screen_rect.left_bottom(),
                        (screen_rect.left_bottom() - Pos2::new(-40., 40.)).to_pos2(),
                    ),
                    0.0,
                    Color32::from_rgb(50, 50, 50),
                    egui::Stroke::NONE,
                    egui::StrokeKind::Inside,
                );
                painter.circle(
                    (screen_rect.left_bottom() - Pos2::new(-20., 20.)).to_pos2(),
                    self.brush.size,
                    self.colour_wheel.current,
                    egui::Stroke::NONE,
                );

                for shape in &self.shapes {
                    shape.draw(&painter);
                }

                if let Some(current_shape) = &self.current_shape {
                    current_shape.draw(&painter);
                }

                match (ctrl, scroll) {
                    (false, scroll) if scroll > 0.0 => self.colour_wheel.next(),
                    (false, scroll) if scroll < 0.0 => self.colour_wheel.prev(),
                    (true, scroll) if scroll > 0.0 => self.brush.enlarge(),
                    (true, scroll) if scroll < 0.0 => self.brush.shrink(),
                    _ => {}
                }

                if ui.input(|i| i.pointer.primary_down()) {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        match self.current_shape.as_mut() {
                            Some(current_shape) => {
                                current_shape.update_pos(pos);
                            }
                            None if ctrl => {
                                self.current_shape = Some(Shape::Arrow(Arrow {
                                    start: pos,
                                    end: pos,
                                    colour: self.colour_wheel.current,
                                    size: self.brush.size,
                                }))
                            }
                            None if alt => {
                                self.current_shape = Some(Shape::Line(Line {
                                    start: pos,
                                    end: pos,
                                    colour: self.colour_wheel.current,
                                    size: self.brush.size,
                                }))
                            }
                            None => {
                                self.current_shape = Some(Shape::Stroke(Stroke {
                                    points: vec![pos],
                                    colour: self.colour_wheel.current,
                                    size: self.brush.size,
                                }))
                            }
                        }
                    }
                } else if let Some(current_shape) = self.current_shape.take() {
                    self.shapes.push(current_shape);
                }

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ctrl && ui.input(|i| i.key_pressed(egui::Key::Z)) {
                    self.shapes.pop();
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top(),
        ..Default::default()
    };
    eframe::run_native(
        "overpaint",
        options,
        Box::new(|_cc| Ok(Box::new(SketchApp::default()))),
    )
}
