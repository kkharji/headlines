mod hyperlink;
pub(crate) use hyperlink::*;

macro_rules! Layout {
    ($ui:ident, $dir:ident, $closure:expr) => {
        $ui.with_layout(Layout::$dir(), $closure);
    };
}

macro_rules! Space {
    ($ui:ident) => {
        Space!($ui, 10.)
    };

    ($ui:ident, $size:literal) => {
        $ui.add_space($size);
    };
}

macro_rules! Label {
    (text: $text:expr, style: $style:expr, $ui:ident) => {
        $ui.label(RichText::new($text).text_style($style))
    };
}

macro_rules! Button {
    (text: $text:expr, style: $style:expr, $ui:ident) => {
        $ui.button(RichText::new($text).text_style($style))
    };
}

pub(crate) use {Button, Label, Layout, Space};
