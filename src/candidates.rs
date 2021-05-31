use crate::history::VisitedSite;
use crate::preferences::SiteWithJavascriptEnabled;
use std::collections::HashMap;

pub fn sites_to_remove(
    safelist: Vec<SiteWithJavascriptEnabled>,
    history: Vec<VisitedSite>,
    threshold: u64,
) -> Vec<SiteWithJavascriptEnabled> {
    history
        .into_iter()
        .zip(safelist.into_iter())
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
            if visits >= threshold {
                Some(top_level_site)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::candidates::sites_to_remove;
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
        }
    }

    #[test]
    fn sites_on_safelist_below_threshold() {
        let visited = vec![visited("https://one.com/some-site", 10)];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 10);

        assert_eq!(remove_from_safelist, vec![safelisted("https://one.com")]);
    }

    #[test]
    fn ignore_sites_on_safelist_above_threshold() {
        let visited = vec![visited("https://one.com/some-site", 10)];
        let safelist = vec![safelisted("https://one.com")];

        let remove_from_safelist = sites_to_remove(safelist, visited, 11);

        assert!(remove_from_safelist.is_empty());
    }
}
