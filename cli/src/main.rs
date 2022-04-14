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
            .query(["announce that"])
            .headlines()
            .limit(5)
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
            .query(["Elon Musk"])
            .everything()
            .limit(10)
            .scope([scope::content()])
            .language(lang::en())
            .sources(["bbc-news", "engadget"])
            .run()
            .unwrap()
            .render();
    }
}
