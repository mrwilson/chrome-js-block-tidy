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
}
