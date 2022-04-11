use clap::StructOpt;
use colour as c;
use headlines::{ArticleCollection, NewsApi, Result};

fn render(articles: &ArticleCollection) {
    c::grey_ln!("---------------------------------------------------------------------");
    c::white_ln!("NewsApi Result:");
    c::grey_ln!("---------------------------------------------------------------------");
    for article in articles.iter() {
        c::dark_green_ln!("> {}", article.title);
        c::blue_ln!("  {}", article.url)
    }
}

fn main() -> Result<()> {
    let api = NewsApi::parse();
    let articles = api.request()?;
    render(&articles);
    Ok(())
}

// #[derive(clap::Parser)]
// struct Args {
//     #[clap(subcommand)]
//     command: Commands,
// }

// #[derive(clap::Subcommand)]
// enum Commands {
//     /// Query NEWSAPI.
//     Query(NewsApi),
//     /// Get valid sources from NEWSAPI.
//     Sources,
// }
