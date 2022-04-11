use eframe::epaint::Vec2;
use eframe::{run_native, NativeOptions};

pub mod app;
mod components;
mod fonts;
mod macros;
mod pages;
mod state;
mod style;

fn main() {
    tracing_subscriber::fmt().init();
    #[allow(unused_mut)]
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(740., 960.));

    run_native(Box::new(app::App::default()), options);
}
