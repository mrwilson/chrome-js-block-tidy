use crate::chrome;
use crate::history::VisitedSite;
use crate::preferences::SiteWithJavascriptEnabled;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn sites_to_remove(
    safelist: Vec<SiteWithJavascriptEnabled>,
    history: Vec<VisitedSite>,
    threshold: u64,
) -> Vec<SiteWithJavascriptEnabled> {
    history
        .into_iter()
        .flat_map(|visited| {
            safelist
                .clone()
                .into_iter()
                .map(move |safelisted_site| (visited.clone(), safelisted_site))
        })
        .filter_map(|(visited, safelist)| {
            if visited.url.starts_with(&safelist.url) {
                Some((safelist, visited.visits))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, (site, visits)| {
            *map.entry(site).or_insert(0) += visits;
            map
        })
        .into_iter()
        .filter_map(|(top_level_site, visits)| {
            if visits < threshold {
                Some(top_level_site)
            } else {
                None
            }
        })
        .collect()
}

pub fn remove_sites(sites_to_remove: Vec<SiteWithJavascriptEnabled>) {
    let preferences_json = std::fs::read_to_string(&chrome::preferences()).unwrap();

    let new_json: String = remove_entries_from_json(sites_to_remove, &preferences_json);

    let mut writer = OpenOptions::new()
        .write(true)
        .open(&chrome::preferences())
        .unwrap();

    writer.set_len(0).unwrap();
    writer.write(new_json.as_bytes()).unwrap();
}

fn remove_entries_from_json(
    sites_to_remove: Vec<SiteWithJavascriptEnabled>,
    preferences_json: &String,
) -> String {
    let mut preferences: Value = serde_json::from_str(&preferences_json).unwrap();

    let javascript_exceptions: &mut Map<String, Value> = exceptions(&mut preferences).unwrap();

    for site in sites_to_remove {
        println!("Removing {0}", site.url);
        javascript_exceptions.remove(&site.json_key);
    }

    serde_json::to_string_pretty(&preferences).unwrap()
}

fn exceptions(preferences: &mut Value) -> Option<&mut Map<String, Value>> {
    preferences
        .get_mut("profile")?
        .get_mut("content_settings")?
        .get_mut("exceptions")?
        .get_mut("javascript")?
        .as_object_mut()
}

#[cfg(test)]
mod test {
    use crate::candidates::{remove_entries_from_json, sites_to_remove};
    use crate::history::VisitedSite;
    use crate::preferences::SiteWithJavascriptEnabled;

    fn visited(url: &str, visit: u64) -> VisitedSite {
        VisitedSite {
            url: String::from(url),
            visits: visit,
        }
    }

    fn safelisted(url: &str) -> SiteWithJavascriptEnabled {
        SiteWithJavascriptEnabled {
            url: String::from(url),
            json_key: String::from("some_key"),
        }
    }

    #[test]
    fn sites_on_safelist_below_threshold() {
        let visited = vec![visited("https://one.com/some-site", 1)];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 2);

        assert_eq!(remove_from_safelist, vec![safelisted("https://one.com")]);
    }

    #[test]
    fn ignore_sites_on_safelist_above_threshold() {
        let visited = vec![visited("https://one.com/some-site", 2)];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 1);

        assert!(remove_from_safelist.is_empty());
    }

    #[test]
    fn sites_on_safelist_with_multiple_visits_below_threshold() {
        let visited = vec![
            visited("https://one.com/some-site", 1),
            visited("https://one.com/some-other-site", 1),
        ];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 3);

        assert_eq!(remove_from_safelist, vec![safelisted("https://one.com")]);
    }

    #[test]
    fn ignore_sites_on_safelist_with_multiple_visits_above_threshold() {
        let visited = vec![
            visited("https://one.com/some-site", 2),
            visited("https://one.com/some-other-site", 2),
        ];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 3);

        assert!(remove_from_safelist.is_empty());
    }

    #[test]
    fn remove_entry_from_preferences_json() {
        let sites_to_remove = vec![SiteWithJavascriptEnabled {
            url: String::from("http://google.com"),
            json_key: String::from("https://www.google.com:443,*"),
        }];

        let old_file = r#"
        {
            "profile": { "content_settings": { "exceptions": { "javascript": {
                "https://www.google.com:443,*": {
                    "expiration": "0",
                    "last_modified": "16188888000000",
                    "model": 0,
                    "setting": 1
                }
            }}}}
        }"#
        .to_owned();

        assert!(!remove_entries_from_json(sites_to_remove, &old_file)
            .contains("https://www.google.com:443,*"));
    }
}
