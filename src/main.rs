use eframe::egui::ViewportBuilder;
use overpaint::OverpaintApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: setup_viewport(),
        ..Default::default()
    };
    eframe::run_native(
        "overpaint",
        options,
        Box::new(|_cc| Ok(Box::new(OverpaintApp::default()))),
    )
}

#[cfg(target_os = "macos")]
fn setup_viewport() -> ViewportBuilder {
    use display_info::DisplayInfo;
    use eframe::egui::{Pos2, Vec2};

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
