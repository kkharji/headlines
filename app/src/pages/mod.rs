mod headlines;
mod settings;
use eframe::egui::TextStyle::Body;
use eframe::egui::{Response, RichText, Ui};
use strum::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr, Debug, PartialEq, Default)]
pub enum Page {
    #[default]
    Headlines,
    Search,
    Settings,
    Favored,
}

impl Page {
    /// Get UI Button
    ///
    /// if toggle is true then create button toggle between:
    ///
    /// Headlines and Search headline
    pub fn render_button(&mut self, ui: &mut Ui, toggle: bool) -> Response {
        let label = RichText::new(self.icon(toggle)).text_style(Body);
        let hover = self.hover_text(toggle);
        let button = ui.button(label).on_hover_text(hover);

        if toggle && button.clicked() {
            self.toggle();
        }

        button
    }

    /// Get Page Icon.
    ///
    /// If second arg is true, then consider only:
    /// Headlines and Search icon
    pub fn icon(&self, toggle: bool) -> &str {
        let main = "";
        match self {
            Self::Search => main,
            Self::Headlines => "",
            Self::Settings | Self::Favored if toggle => main,
            Self::Favored => "",
            Self::Settings => "",
        }
    }

    /// Convenient function to toggle between pages:
    /// Headlines, Search.
    pub fn toggle(&mut self) {
        if self.is_headlines() {
            *self = Self::Search
        } else {
            *self = Self::Headlines
        }
    }

    /// Get Page Hover Text
    ///
    /// If second arg is true, then it will consider only:
    /// Headlines and Search headline
    pub fn hover_text(&self, toggle: bool) -> &str {
        let default = Self::Headlines.as_ref();
        match self {
            Self::Search => default,
            Self::Headlines => Self::Search.as_ref(),
            Self::Settings | Self::Favored if toggle => default,
            _ => self.as_ref(),
        }
    }

    /// Return true if the page is Search
    pub fn is_search(&self) -> bool {
        matches!(self, Self::Search)
    }

    /// Return true if the page is Headlines
    pub fn is_headlines(&self) -> bool {
        matches!(self, Self::Headlines)
    }

    /// Return true if the page is Settings
    pub fn is_settings(&self) -> bool {
        matches!(self, Self::Settings)
    }
}
