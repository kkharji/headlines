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
//
macro_rules! _Label {
    (text: $text:expr, style: $style:ident, $ui:ident) => {
        $ui.label(eframe::egui::RichText::new($text).text_style(eframe::egui::TextStyle::$style))
    };
    (text: $text:expr, style: $style:expr, $ui:ident) => {
        $ui.label(eframe::egui::RichText::new($text).text_style($style))
    };
}

macro_rules! Label {
    ($ui:ident, $text:expr, $style:ident) => {
        _Label!(text: $text, style: $style, $ui)
    };

    ($text:expr, $style:ident, $ui:ident) => {
        _Label!(text: $text, style: $style, $ui)
    };

    ($ui:ident, $text:expr) => {
        _Label!(text: $text, style: Body, $ui)
    };

    ($text:expr, $ui:ident) => {
        _Label!(text: $text, style: Body, $ui)
    };

    ($text:expr, $style:ident, $ui:expr) => {
        _Label!(text: $text, style: $style, $ui)
    };

    ($ui:ident, $text:expr, $style:expr) => {
        _Label!(text: $text, style: $style, $ui)
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
    Button, Label, Layout, Separator, Space, TopBottomPanel, VerticalCentered, _Label,
};
