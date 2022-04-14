mod hyperlink;
pub(crate) use hyperlink::*;

// mod text;
// pub(crate) use text::*;

macro_rules! Layout {
    ($ui:ident, $dir:ident, $closure:expr) => {
        $ui.with_layout(Layout::$dir(), $closure);
    };
}

macro_rules! Space {
    ($ui:ident) => {
        Space!(10., $ui)
    };

    ($size:expr, $ui:ident) => {
        $ui.add_space($size)
    };
}

macro_rules! Separator {
    ($ui:ident) => {
        $ui.add(eframe::egui::Separator::default())
    };

    ($ui:ident, $size:expr) => {
        $ui.add(eframe::egui::Separator::default().spacing($size))
    };

    ($size:expr, $ui:ident) => {
        Separator!($ui, $size)
    };
}

macro_rules! Label {
    ($text:expr, $style:ident, $ui:ident) => {
        $ui.label(eframe::egui::RichText::new($text).text_style(eframe::egui::TextStyle::$style))
    };

    ($text:expr, $ui:ident) => {
        $ui.label($text)
    };

    // ($text:expr, $style:expr, $ui:ident) => {
    //     $ui.label(eframe::egui::RichText::new($text).text_style($style))
    // };
    ($ui:ident, $text:expr, $style:ident) => {
        Label!($text, $style, $ui)
    };

    ($text:expr, $style:ident, $ui:ident) => {
        Label!($text, $style, $ui)
    };

    ($ui:ident, $text:expr) => {
        Label!($text, Body, $ui)
    };

    ($text:expr, $ui:ident) => {
        Label!($text, Body, $ui)
    };

    ($text:expr, $style:ident, $ui:expr) => {
        Label!($text, $style, $ui)
    };

    ($ui:ident, $text:expr, $style:expr) => {
        Label!($text, $style, $ui)
    };

    ($text:expr, $color:ident, $style:ident, $ui:ident) => {
        $ui.colored_label($color, eframe::egui::RichText::new($text).$style())
    };

    ($style:ident, $text:expr,  $ui:ident) => {
        Label!($text, $style, $ui)
    };
}
macro_rules! UiWithLayout {
    ($ui:ident, $dir:ident, $size:literal, $cb:expr) => {
        $ui.allocate_ui_with_layout(
            eframe::epaint::Vec2::new($ui.available_width(), $size),
            eframe::egui::Layout::$dir(),
            $cb,
        )
    };
}

macro_rules! Button {
    ($text:expr, $style:expr, $ui:ident) => {
        $ui.button(eframe::egui::RichText::new($text).text_style($style))
    };
}

macro_rules! VerticalCentered {
    ($ui:ident, $cb:expr) => {
        $ui.vertical_centered($cb);
    };
    ($cb:expr ,$ui:ident) => {
        VerticalCentered!($ui, $cb)
    };
}

macro_rules! TopBottomPanel {
    ($ctx:ident, $cb:expr) => {
        eframe::egui::TopBottomPanel::bottom("footer").show($ctx, $cb)
    };
    ($cb:expr ,$ctx:ident) => {
        TopBottomPanel!($ctx, $cb)
    };
}

pub(crate) use {
    Button, Label, Layout, Separator, Space, TopBottomPanel, UiWithLayout, VerticalCentered,
};
