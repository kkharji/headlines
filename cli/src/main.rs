use headlines::client::request;

fn main() {
    request::from_cli_args().run().unwrap().render();
}

#[cfg(test)]
mod tests {
    use super::*;
    use headlines::article::{lang, scope};

    #[test]
    fn case1() {
        /*
        Find top 5 articles that
            - contains "announce that" in it's content/body
            - written in english
            - published in Hacker News
        */
        request()
            .headlines()
            .limit(5)
            .query(&["announce that"])
            .scope(&[scope::content()])
            .language(lang::en())
            // .sources(&["hacker-news"])
            .run()
            .unwrap()
            .render();
    }
}
