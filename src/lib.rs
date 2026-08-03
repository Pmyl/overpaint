use eframe::egui::{
    Align2, CentralPanel, Color32, Context, CursorIcon, Event, FontId, Frame, Key, Pos2, Rect,
    Stroke as EguiStroke, Ui, ViewportCommand, Visuals,
};

use crate::{
    features::{
        brush::Brush,
        colour_wheel::{ColourWheel, ColourWheelUi},
        debug::draw_bounding_rect,
        history::HistoryEvent,
        selection::RectSelection,
    },
    shapes::{Shape, arrow::Arrow, line::Line, stroke::Stroke, text::Text},
};

mod features;
mod geometry;
mod shapes;

pub struct OverpaintApp {
    // Used for animations, updates every frame, wraps around on overflow
    app_counter: usize,
    debug_mode: bool,

    shapes: Vec<Shape>,
    current_shape: Option<Shape>,
    colour_wheel: ColourWheel,
    brush: Brush,

    colour_wheel_ui: ColourWheelUi,

    history: Vec<HistoryEvent>,
    previous_mouse_position: Pos2,
    mouse_position: Pos2,
    selection: Option<RectSelection>,
}

impl Default for OverpaintApp {
    fn default() -> Self {
        Self {
            app_counter: 0,
            debug_mode: false,

            shapes: Vec::new(),
            current_shape: None,
            colour_wheel: ColourWheel::default(),
            brush: Brush::default(),
            colour_wheel_ui: ColourWheelUi::default(),

            history: Vec::new(),
            previous_mouse_position: Pos2::ZERO,
            mouse_position: Pos2::ZERO,
            selection: None,
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

            for shape in &self.shapes {
                shape.draw(painter);
            }

            if let Some(current_shape) = &self.current_shape {
                current_shape.draw(painter);
            } else if let Some(selection) = &self.selection {
                selection.draw(painter, &self.shapes, self.app_counter);
                ui.request_repaint();
            } else {
                painter.circle(
                    self.mouse_position,
                    self.brush.size / 2.0,
                    self.colour_wheel.current,
                    EguiStroke::NONE,
                );
            }

            if self.debug_mode {
                painter.text(
                    Pos2::ZERO,
                    Align2::LEFT_TOP,
                    "DEBUG MODE: ON",
                    FontId::default(),
                    Color32::RED,
                );

                for shape in &self.shapes {
                    draw_bounding_rect(
                        painter,
                        shape,
                        self.previous_mouse_position,
                        self.mouse_position,
                    );
                }

                if let Some(current_shape) = &self.current_shape {
                    draw_bounding_rect(
                        painter,
                        current_shape,
                        self.previous_mouse_position,
                        self.mouse_position,
                    );
                }
            }

            self.colour_wheel_ui.draw(
                painter,
                Pos2::new(0.0, ui.input(|ui| ui.content_rect()).height()),
                &self.colour_wheel,
            );
        });
    }

    fn logic(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.app_counter = self.app_counter.wrapping_add(1);

        let InteractionsInfo {
            shift,
            ctrl,
            alt,
            escape,
            undo,
            scroll,
            f1,
            mouse_position,
            text_events,
            has_text_events,
            mouse_primary_pressed,
            mouse_primary_released,
            mouse_secondary_held,
        } = extract_interactions_info(ctx);

        self.previous_mouse_position = self.mouse_position;
        self.mouse_position = mouse_position;

        if let Some(current_shape) = self.current_shape.as_mut() {
            current_shape.update(
                self.mouse_position,
                self.brush.size,
                self.colour_wheel.current,
                ctx,
            );
        }

        if f1 {
            self.debug_mode = !self.debug_mode;
        }

        if scroll != 0.0 {
            match ctrl {
                false if scroll > 0.0 => self.colour_wheel.next(),
                false if scroll < 0.0 => self.colour_wheel.prev(),
                true if scroll > 0.0 => self.brush.enlarge(),
                true if scroll < 0.0 => self.brush.shrink(),
                _ => {}
            }
        }

        if mouse_secondary_held {
            self.handle_eraser(
                self.brush.size,
                self.previous_mouse_position,
                self.mouse_position,
            );
            return;
        }

        if undo {
            match self.history.pop() {
                Some(HistoryEvent::Add) | None => {
                    self.shapes.pop();
                }
                Some(HistoryEvent::Remove(shape)) => {
                    self.shapes.push(shape);
                }
                Some(HistoryEvent::AddSelection) => {
                    self.selection
                        .take()
                        .unwrap()
                        .reset_shapes(&mut self.shapes);
                }
                Some(HistoryEvent::RemoveSelection(selection)) => {
                    self.selection = Some(selection);
                }
            }
            return;
        }

        match self.selection.as_mut() {
            Some(selection) if mouse_primary_released => {
                selection.complete(self.mouse_position, &mut self.shapes);
                if !selection.shapes_indices.is_empty() {
                    self.history.push(HistoryEvent::AddSelection);
                } else {
                    self.selection.take();
                }
                return;
            }
            Some(_) if mouse_primary_pressed => {
                let mut selection = self.selection.take().unwrap();
                selection.update(self.mouse_position, &mut self.shapes);
                self.history.push(HistoryEvent::RemoveSelection(selection));
                return;
            }
            Some(_) if escape => {
                self.selection
                    .take()
                    .unwrap()
                    .reset_shapes(&mut self.shapes);
                return;
            }
            Some(selection) => {
                selection.update(self.mouse_position, &mut self.shapes);
                return;
            }
            None if shift && mouse_primary_pressed => {
                self.selection = Some(RectSelection {
                    is_selecting: true,
                    anchor: self.mouse_position,
                    origin: self.mouse_position,
                    rect: Rect {
                        min: self.mouse_position,
                        max: self.mouse_position,
                    },
                    shapes_indices: vec![],
                });
                return;
            }
            _ => {}
        }

        if let Some(current_shape) = self.current_shape.as_mut() {
            match current_shape {
                Shape::Text(text_shape) if mouse_primary_pressed => {
                    self.shapes.push(self.current_shape.take().unwrap());
                    self.history.push(HistoryEvent::Add);
                }
                Shape::Text(text_shape) if has_text_events => {
                    let new_text = apply_text_events(&text_events, text_shape.text.clone(), ctrl);

                    if !new_text.is_empty() {
                        text_shape.set_text(new_text);
                    } else {
                        self.current_shape.take();
                    }
                }
                Shape::Text(_) if escape => {
                    self.current_shape.take();
                    return;
                }
                _ => {
                    if mouse_primary_released {
                        self.shapes.push(self.current_shape.take().unwrap());
                        self.history.push(HistoryEvent::Add);
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
                let new_text = apply_text_events(&text_events, String::new(), ctrl);
                if !new_text.is_empty() {
                    self.current_shape = Some(Shape::Text(Text::new(
                        new_text,
                        mouse_position,
                        self.colour_wheel.current,
                        self.brush.size,
                        ctx,
                    )));
                }
            }
        }

        if escape {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }
}

impl OverpaintApp {
    fn handle_eraser(
        &mut self,
        thickness: f32,
        previous_mouse_position: Pos2,
        mouse_position: Pos2,
    ) {
        let mut removed_shapes_events: Vec<HistoryEvent> = self
            .shapes
            .extract_if(.., |shape| {
                shape.in_bounding_rect(thickness, previous_mouse_position, mouse_position)
                    && shape.touches(thickness, previous_mouse_position, mouse_position)
            })
            .map(|shape| HistoryEvent::Remove(shape))
            .collect();

        self.history.append(&mut removed_shapes_events);
    }
}

fn extract_interactions_info(ctx: &Context) -> InteractionsInfo {
    ctx.input(|i| {
        let shift = i.modifiers.shift;
        let ctrl = i.modifiers.ctrl;
        let alt = i.modifiers.alt;
        let escape = i.key_pressed(Key::Escape);
        let undo = ctrl && i.key_pressed(Key::Z);
        let f1 = i.key_pressed(Key::F1);
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
        let mouse_secondary_held = i.pointer.secondary_down();

        InteractionsInfo {
            shift,
            ctrl,
            alt,
            escape,
            undo,
            f1,
            scroll,
            mouse_position,
            text_events,
            has_text_events,
            mouse_primary_pressed,
            mouse_primary_released,
            mouse_secondary_held,
        }
    })
}

fn apply_text_events(text_events: &[Event], mut current_text: String, ctrl: bool) -> String {
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
}

pub struct InteractionsInfo {
    shift: bool,
    ctrl: bool,
    alt: bool,
    escape: bool,
    undo: bool,
    f1: bool,
    scroll: f32,
    mouse_position: Pos2,
    text_events: Vec<Event>,
    has_text_events: bool,
    mouse_primary_pressed: bool,
    mouse_primary_released: bool,
    mouse_secondary_held: bool,
}
