use colour as c;
use newsapp::{api, color_eyre_install, Result};

fn main() -> Result<()> {
    color_eyre_install()?;
    let articles = api::query_everything(&["bitcoin"]).request()?;

    for article in articles.iter() {
        c::dark_green_ln!("> {}", article.title);
        c::blue_ln!("  {}", article.url)
    }

    Ok(())
}
