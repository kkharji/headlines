#![allow(dead_code)]
pub mod colors;

use crate::App;
use eframe::egui::style::{Margin, WidgetVisuals, Widgets};
use eframe::egui::{Context, Frame, RichText, TextStyle, Ui, Visuals};
use eframe::epaint::{Color32 as C, Rounding, Stroke};
use std::default::default;

// Styles
pub const fn padding() -> f32 {
    5.0
}
pub const fn margin() -> f32 {
    30.0
}

pub fn is_dark_mode(ui: &Ui) -> bool {
    ui.style().visuals.dark_mode
}

#[inline(always)]
pub fn text(text: &str) -> RichText {
    RichText::new(text)
}

#[inline(always)]
pub fn heading<S: Into<String>>(text: S) -> RichText {
    RichText::new(text).text_style(TextStyle::Heading)
}

impl App {
    pub fn configure_styles(&mut self, ctx: &Context, toggle_mode: bool) {
        if toggle_mode {
            self.config.mode.toggle();
        };
        let mut style = (*ctx.style()).clone();
        let dark_mode = self.config.mode.is_dark();
        style.visuals = Visuals {
            dark_mode,
            hyperlink_color: self.red(),
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    // Window Background,
                    bg_fill: self.background(),
                    // Separators, indentation lines, windows outlines
                    bg_stroke: Stroke::new(1.0, C::from_gray(if dark_mode { 60 } else { 190 })),
                    // Normal text color
                    fg_stroke: Stroke::new(1.0, C::from_gray(if dark_mode { 140 } else { 80 })),
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                ..self.get_widget_default(dark_mode)
            },
            ..self.get_visual_default(dark_mode)
        };
        ctx.set_style(style);
    }

    fn get_visual_default(&self, dark_mode: bool) -> Visuals {
        if dark_mode {
            tracing::trace!("Swtich dark mode.");
            Visuals::dark()
        } else {
            tracing::trace!("Swtich light mode.");
            Visuals::light()
        }
    }

    fn get_widget_default(&self, dark_mode: bool) -> Widgets {
        if dark_mode {
            Widgets::dark()
        } else {
            Widgets::light()
        }
    }

    #[allow(dead_code)]
    /// FIXME: sets very dark background or reset it
    pub fn get_default_frame(&self) -> Frame {
        Frame {
            margin: Margin {
                left: margin(),
                right: margin(),
                top: margin(),
                bottom: margin(),
            },
            ..default()
        }
    }
}
