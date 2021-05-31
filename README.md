# chrome-js-block-tidy

A tool to manage exceptions when running Chrome without Javascript enabled.

It calculates which URLs on the JavaScript safelist haven't been visited much recently and could possibly be removed. For example:

```bash
$ chrome-js-block-tidy --threshold 20 --days-ago 365

https://stackoverflow.com
https://google.com
https://blog.probablyfine.co.uk
```

TODO: Automate editing the Chrome Preferences JSON to automatically remove them.

## Usage

```
$ chrome-js-block-tidy --help

chrome-js-block-tidy 0.1.0
A tool to manage exceptions when running Chrome without Javascript enabled

USAGE:
    chrome-js-block-tidy --days-ago <days-ago> --threshold <threshold>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --days-ago <days-ago>      Only count visits to sites in this period [default: 7]
        --threshold <threshold>    Any site with fewer than this number of visits should be removed [default: 10]
```