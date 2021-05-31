use clap::{App, Arg};

mod candidates;
mod history;
mod preferences;

fn main() {
    let matches = App::new("chrome-js-block-tidy")
        .version("0.1.0")
        .about("A tool to manage exceptions when running Chrome without Javascript enabled")
        .arg(
            Arg::with_name("threshold")
                .required(true)
                .long("threshold")
                .help("Any site with fewer than this number of visits should be removed")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("days-ago")
                .required(true)
                .long("days-ago")
                .help("Only count visits to sites in this period")
                .default_value("7"),
        )
        .get_matches();

    let threshold = match matches.value_of("threshold").unwrap().parse::<u64>() {
        Ok(n) => n,
        Err(_) => 10,
    };

    let days_ago = match matches.value_of("days-ago").unwrap().parse::<u16>() {
        Ok(n) => n,
        Err(_) => 7,
    };

    let safelist = preferences::sites_with_js_enabled();
    let visited_sites = history::sites_visited_recently(days_ago);

    println!(
        "{:?}",
        candidates::sites_to_remove(safelist, visited_sites, threshold)
    );
}
