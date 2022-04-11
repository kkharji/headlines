use eframe::egui::{Context, FontData, FontDefinitions, TextStyle};
use eframe::epaint::{FontFamily, FontId};

pub(crate) fn configure(ctx: &Context) {
    configure_fonts(ctx);
    configure_text_styles(ctx);
}

fn configure_fonts(ctx: &Context) {
    // Start with the default fonts set
    let mut fonts = FontDefinitions::default();
    // Insert custom font
    fonts.font_data.insert(
        "MesloLGS".into(),
        FontData::from_static(include_bytes!("../MesloLGL_NF_Regular.ttf")),
    );
    // NOTE: Prioritize font for proportional text:
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "MesloLGS".into());
    // .insert(TextStyle::Heading, (FontFamily::Proportional, 35))
    // NOTE: Load font
    ctx.set_fonts(fonts);
}

fn configure_text_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    // Note: Set the size of different text styles.
    style.text_styles = [
        // (title(), { FontId::new(32.0, FontFamily::Proportional) }),
        (inner_link(), {
            FontId::new(25.0, FontFamily::Proportional)
        }),
        (TextStyle::Heading, {
            FontId::new(23.0, FontFamily::Proportional)
        }),
        (TextStyle::Body, {
            FontId::new(18.0, FontFamily::Proportional)
        }),
        (TextStyle::Monospace, {
            FontId::new(14.0, FontFamily::Monospace)
        }),
        (TextStyle::Button, {
            FontId::new(14.0, FontFamily::Proportional)
        }),
        (TextStyle::Small, {
            FontId::new(10.0, FontFamily::Proportional)
        }),
    ]
    .into();
    ctx.set_style(style);
}

#[inline]
pub fn inner_link() -> TextStyle {
    TextStyle::Name("NavPageLink".into())
}

#[allow(dead_code)]
pub fn title() -> TextStyle {
    TextStyle::Name("Title".into())
}
