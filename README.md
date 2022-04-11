# Headlines

## Lib

### Usage Examples
```rust 

/*
Find top 5 articles that
    - contains "announce that"
    - published in thehill and theverge
*/
request()
    .headlines()
    .limit(5)
    .query(&["announce that"])
    .run()
    .unwrap()

/*
Find 10 articles that
    - contains "Elon Musk" in content
    - in english language
    - published in bbc-news and engadget
*/
request()
    .everything()
    .limit(10)
    .query(&["Elon Musk"])
    .scope(&[scope::content()])
    .language(lang::en())
    .sources(&["bbc-news", "engadget"])
    .run()
    .unwrap()
```

## CLI:

```
USAGE:
    headlines_cli [OPTIONS] [QUERY]...

ARGS:
    <QUERY>...
            NewsApi Search Query:

            Must appear (Eg: +bitcoin). Must not appear (Eg: -bitcoin). AND / OR / NOT keywords:
            (Eg: crypto, AND, (ethereum, OR, litecoin), NOT, bitcoin)

OPTIONS:
    -c, --category <CATEGORY>
    -C, --country <COUNTRY>
    -d, --domains <DOMAINS>
    -e, --endpoint <ENDPOINT>
    -E, --exclude-domains <EXCLUDE_DOMAINS>
    -f, --from <FROM>
    -h, --help
    -i, --in <SOURCES>
    -l, --limit <PAGE_SIZE>
    -L, --language <LANGUAGE>
    -P, --page <PAGE>
    -s, --scope <SEARCHIN>
    -t, --to <TO>
```
