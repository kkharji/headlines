#![allow(dead_code)]
use eframe::egui::Ui;
use eframe::epaint::Color32;

// COLORS
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub const TRANSPARENT: Color32 = Color32::TRANSPARENT;
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const DARK_GRAY: Color32 = Color32::from_rgb(96, 96, 96);
pub const GRAY: Color32 = Color32::from_rgb(160, 160, 160);
pub const LIGHT_GRAY: Color32 = Color32::from_rgb(220, 220, 220);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub const BROWN: Color32 = Color32::from_rgb(165, 42, 42);
pub const DARK_RED: Color32 = Color32::from_rgb(0x8B, 0, 0);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub const LIGHT_RED: Color32 = Color32::from_rgb(255, 128, 128);

pub const YELLOW: Color32 = Color32::from_rgb(255, 255, 0);
pub const LIGHT_YELLOW: Color32 = Color32::from_rgb(255, 255, 0xE0);
pub const KHAKI: Color32 = Color32::from_rgb(240, 230, 140);

pub const DARK_GREEN: Color32 = Color32::from_rgb(0, 0x64, 0);
pub const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
pub const LIGHT_GREEN: Color32 = Color32::from_rgb(0x90, 0xEE, 0x90);

pub const DARK_BLUE: Color32 = Color32::from_rgb(0, 0, 0x8B);
pub const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
pub const LIGHT_BLUE: Color32 = Color32::from_rgb(0xAD, 0xD8, 0xE6);

pub const GOLD: Color32 = Color32::from_rgb(255, 215, 0);

// Styles
pub const PADDING: f32 = 5.0;

pub fn is_dark_mode(ui: &Ui) -> bool {
    ui.style().visuals.dark_mode
}

pub fn update_style(ui: &mut Ui) {
    let is_dark_mode = is_dark_mode(ui);
    ui.style_mut().visuals.hyperlink_color = if is_dark_mode { CYAN } else { RED };
}
