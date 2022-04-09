use clap::StructOpt;
use colour as c;
use newsapp::{ArticleCollection, NewsApi, Result};

fn render(articles: &ArticleCollection) {
    c::grey_ln!("---------------------------------------------------------------------");
    c::white_ln!("NewsApi Result:");
    c::grey_ln!("---------------------------------------------------------------------");
    for article in articles.iter() {
        c::dark_green_ln!("> {}", article.title);
        c::blue_ln!("  {}", article.url)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api = NewsApi::parse();
    let articles = api.request().await?;
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
