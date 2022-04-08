use colour as c;
use newsapp::api::{NewsApi, NewsApiEndpoint};
use newsapp::{color_eyre_install, ArticleCollection, NewsApiCache, Result};

fn main() -> Result<()> {
    color_eyre_install()?;
    let mut cache = NewsApiCache::default();
    let articles = NewsApi::new(&mut cache)
        .query(&["rust", "api"])
        .endpoint(NewsApiEndpoint::TopHeadlines)
        .request()?;

    render(&articles);

    cache.persist()?;

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
