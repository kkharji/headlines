macro_rules! Hyperlink {
    // ui, "title", "https://...", style
    ($ui:ident, $label:literal, $link:expr, $style:ident) => {
        $ui.add(eframe::egui::Hyperlink::from_label_and_url(
            eframe::egui::RichText::new($label).$style(),
            $link,
        ))
    };

    // ui, "title", "https://..."
    ($ui:ident, $label:literal, $link:expr) => {
        Hyperlink!($ui, $label, $link, monospace)
    };

    // "title", "https://...", ui
    ($label:literal, $link:expr, $ui:ident) => {
        Hyperlink!($ui, $label, $link, monospace)
    };

    // ui, "title", "https://...", style
    ($label:literal, $link:expr, $style:ident, $ui:ident) => {
        Hyperlink!($ui, $label, $link, $style)
    };
}

pub(crate) use Hyperlink;
