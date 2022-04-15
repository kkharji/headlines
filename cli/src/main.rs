use headlines::client::request;

fn main() {
    request::from_cli_args().run().unwrap().render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use headlines::article::{lang, scope};

    #[test]
    fn top_announcements() {
        /*
        Find top 5 articles that
            - contains "announce that"
            - published in thehill and theverge
        */
        request()
            .headlines()
            .set_query(["announce that"])
            .set_limit(5)
            .run()
            .unwrap()
            .render();
    }

    #[test]
    fn elon_musk() {
        /*
        Find 10 articles that
            - contains "Elon Musk" in content
            - in english language
            - published in bbc-news and engadget
        */
        request()
            .everything()
            .set_query(["Elon Musk"])
            .set_limit(10)
            .set_scope([scope::content()])
            .set_language(lang::en())
            .set_sources(["bbc-news", "engadget"])
            .run()
            .unwrap()
            .render();
    }
}
