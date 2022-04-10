macro_rules! hyperlink {
    // -------------------------------------------------------------------------
    // ui, "title", "https://...", style
    ($ui:ident, $label:literal, $link:expr, $style:ident) => {
        $ui.add(Hyperlink::from_label_and_url(
            RichText::new($label).$style(),
            $link,
        ));
    };

    // ui, "title", "https://..."
    ($ui:ident, $label:literal, $link:expr) => {
        hyperlink!($ui, $label, $link, monospace);
    };
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // ui => { "title", "https://...", style }
    ($ui:ident => { $label:literal, $link:expr, $style:ident }) => {
        hyperlink!($ui, $label, $link, $style);
    };

    // ui => { "title", "https://..." }
    ($ui:ident => { $label:literal, $link:expr }) => {
        hyperlink!($ui, $label, $link);
    };
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // ui => "title", "https://..."
    ($ui:ident => $label:literal, $link:expr, $style:ident) => {
        hyperlink!($ui, $label, $link, $style);
    };

    // ui => "title", "https://...", style
    ($ui:ident => $label:literal, $link:expr) => {
        hyperlink!($ui, $label, $link);
    };
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // ui "title", "https://...", style
    ($ui:ident $label:literal, $link:expr, $style:ident) => {
        hyperlink!($ui, $label, $link, $style);
    };
    // ui "title", "https://..."
    ($ui:ident $label:literal, $link:expr) => {
        hyperlink!($ui, $label, $link);
    }; // -------------------------------------------------------------------------
}

pub(crate) use hyperlink;
