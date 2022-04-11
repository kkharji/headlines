# Headlines

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
            Article category

    -C, --country <COUNTRY>
            Source country

    -d, --domains <DOMAINS>
            Doamins to search in

    -e, --endpoint <ENDPOINT>
            Type of query: everything, top-headings

            [default: everything]

    -E, --exclude-domains <EXCLUDE_DOMAINS>
            Doamins to exclude

    -f, --from <FROM>
            Date range start

    -h, --help
            Print help information

    -i, --in <SOURCES>
            Source to search in. Max 20 sources

    -l, --limit <PAGE_SIZE>
            Limit number of results to return

            [default: 10]

    -L, --language <LANGUAGE>
            Article language

            [default: en]

    -P, --page <PAGE>
            Page through results

            [default: 1]

    -s, --scope <SEARCHIN>
            Scope to match query in. Valid Options: title, description, content,

    -t, --to <TO>
            Date range end
```
