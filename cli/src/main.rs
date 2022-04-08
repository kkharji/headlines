use colour as c;
use newsapp::api::NewsApi;
use newsapp::{color_eyre_install, ArticleCollection, Result};

fn main() -> Result<()> {
    color_eyre_install()?;
    let articles = NewsApi::new().query(&["bitcoin"]).request()?;

    render(&articles);

    Ok(())
}

fn render(articles: &ArticleCollection) {
    c::grey_ln!("---------------------------------------------------------------------");
    c::white_ln!("NewsApi Result:");
    c::grey_ln!("---------------------------------------------------------------------");
    for article in articles.iter() {
        c::dark_green_ln!("> {}", article.title);
        c::blue_ln!("  {}", article.url)
    }
}
