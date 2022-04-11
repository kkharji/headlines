pub mod query {
    macro_rules! push_opt_vec {
        ($key: literal, $opt: expr, $queries: ident, $sep: literal) => {
            if $opt.is_some() {
                let value = $opt
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .join($sep);
                $queries.push(($key, value))
            }
        };

        ($key: literal, $opt: expr, $queries :ident) => {
            push_opt_vec!($key, $opt, $queries, ", ")
        };
    }

    macro_rules! push_vec {
        ($key: literal, $opt: expr, $queries: ident, $sep: literal) => {
            if !$opt.is_empty() {
                let value = $opt
                    .clone()
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .join($sep);
                $queries.push(($key, value))
            }
        };

        ($key: literal, $opt: expr, $queries :ident) => {
            push_vec!($key, $opt, $queries, ", ")
        };
    }

    macro_rules! push_opt_to_string {
        ($key: literal, $opt: expr, $queries: ident) => {
            if let Some(val) = $opt.as_ref() {
                $queries.push(($key, val.to_string()))
            }
        };
    }

    macro_rules! push_to_string {
        ($key: literal, $opt: expr, $queries: ident) => {
            $queries.push(($key, $opt.to_string()))
        };
    }

    pub(crate) use {push_opt_to_string, push_opt_vec, push_to_string, push_vec};
}
