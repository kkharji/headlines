use crate::components::header;
use crate::pages::all_article;
use crate::state::State;
use eframe::egui::TextStyle::Body;
use eframe::egui::{RichText, Ui};
pub enum Route {
    AllArticles,
    #[allow(dead_code)]
    Home,
    TopArticles,
}

impl Default for Route {
    fn default() -> Self {
        Self::AllArticles
    }
}
impl Route {
    pub fn update(ui: &mut Ui, state: &mut State) {
        match state.current_page {
            Route::AllArticles => all_article(ui, state),
            Route::Home => {
                header(ui, "Home");
            }
            Route::TopArticles => {
                header(ui, "Top");
            }
        }
    }

    pub fn create_buttons(ui: &mut Ui, state: &mut State) {
        // let home = RichText::new("Home").text_style(Body);
        // if ui.button(home).clicked() {
        //     state.set_current_page(Route::Home);
        // };
        ui.add_space(5.);

        let all = RichText::new("All").text_style(Body);
        if ui.button(all).clicked() {
            state.set_current_page(Route::AllArticles);
        };

        let top = RichText::new("Top").text_style(Body);
        if ui.button(top).clicked() {
            state.set_current_page(Route::TopArticles);
        };
    }
}
