# chrome-js-block-tidy

A tool to manage exceptions when running Chrome without Javascript enabled.

It removes urls from the Chrome Javascript safelist which haven't been visited much recently. For example:

```bash
$ chrome-js-block-tidy --minimum-visits 20 --days-ago 365

Removing https://stackoverflow.com
Removing https://google.com
Removing https://blog.probablyfine.co.uk
```

## Usage

```
$ chrome-js-block-tidy --help

chrome-js-block-tidy 0.1.0
A tool to manage exceptions when running Chrome without Javascript enabled

USAGE:
    chrome-js-block-tidy --days-ago <DAYS> --minimum-visits <THRESHOLD>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --days-ago <DAYS>               Only count visits to sites in this period [default: 7]
        --minimum-visits <THRESHOLD>    Any site with fewer than this number of visits should be removed [default: 10]
```