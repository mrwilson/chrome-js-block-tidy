use clap::{App, Arg};

mod candidates;
mod chrome;
mod history;
mod preferences;

fn main() {
    let matches = App::new("chrome-js-block-tidy")
        .version("0.1.0")
        .about("A tool to manage exceptions when running Chrome without Javascript enabled")
        .arg(
            Arg::with_name("THRESHOLD")
                .required(true)
                .long("minimum-visits")
                .help("Any site with fewer than this number of visits should be removed")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("DAYS")
                .required(true)
                .long("days-ago")
                .help("Only count visits to sites in this period")
                .default_value("7"),
        )
        .arg(
            Arg::with_name("DRYRUN")
                .long("dry-run")
                .required(false)
                .takes_value(false)
                .help("Prints the list of sites that would be removed"),
        )
        .get_matches();

    let threshold = match matches.value_of("THRESHOLD").unwrap().parse::<u64>() {
        Ok(n) => n,
        Err(_) => 10,
    };

    let days_ago = match matches.value_of("DAYS").unwrap().parse::<u16>() {
        Ok(n) => n,
        Err(_) => 7,
    };

    let safelist = preferences::sites_with_js_enabled();
    let visited_sites = history::sites_visited_recently(days_ago);

    let candidate_sites = candidates::sites_to_remove(safelist, visited_sites, threshold);

    if matches.is_present("DRYRUN") {
        println!("Sites that would be removed:");
        for site in candidate_sites {
            println!("{0}", site.url);
        }
    } else {
        candidates::remove_sites(candidate_sites);
    }
}
