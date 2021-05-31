use rusqlite::{Connection, OpenFlags};
use std::time::SystemTime;

#[derive(Debug, PartialEq, Clone)]
pub struct VisitedSite {
    pub url: String,
    pub visits: u64,
}

pub fn sites_visited_recently() -> Vec<VisitedSite> {
    let home = std::env::var("HOME").unwrap();
    let db = Connection::open_with_flags(
        home + "/Library/Application Support/Google/Chrome/Default/History",
        OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();

    visited_sites(db)
}

static QUERY: &str = "
select
  urls.url,
  count(*)
from
  urls
  join visits on (visits.url = urls.id)
where
  visit_time >= ?1
group by
  1
";

fn days_ago_in_nanos(days: u16) -> u64 {
    let days_ago = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - (days as u64 * 24 * 60 * 60);

    days_ago * 1000 * 1000
}

fn visited_sites(conn: Connection) -> Vec<VisitedSite> {
    let seven_days_ago = days_ago_in_nanos(7);

    conn.prepare(QUERY)
        .unwrap()
        .query_map([seven_days_ago], |row| {
            Ok(VisitedSite {
                url: row.get(0)?,
                visits: row.get(1)?,
            })
        })
        .unwrap()
        .into_iter()
        .filter_map(|site| site.ok())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::history::{days_ago_in_nanos, visited_sites, VisitedSite};
    use rusqlite::Connection;

    const CREATE_URLS: &str = "CREATE TABLE urls (id INTEGER, url TEXT)";
    const INSERT_URLS: &str = "INSERT INTO urls (id, url) VALUES (?1, ?2)";
    const CREATE_VISITS: &str = "CREATE TABLE visits (url INTEGER, visit_time INTEGER)";
    const INSERT_VISITS: &str = "INSERT INTO visits (url, visit_time) VALUES (?1, ?2)";

    #[test]
    fn read_out_visited_sites() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(CREATE_URLS, []).unwrap();
        conn.execute(INSERT_URLS, ["1", "https://foo.com"]).unwrap();

        conn.execute(CREATE_VISITS, []).unwrap();
        conn.execute(INSERT_VISITS, [1, days_ago_in_nanos(0)])
            .unwrap();

        let visited_sites = visited_sites(conn);

        assert_eq!(
            visited_sites[0],
            VisitedSite {
                url: String::from("https://foo.com"),
                visits: 1
            }
        )
    }
}
