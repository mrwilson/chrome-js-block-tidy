use crate::chrome;
use regex::Regex;
use serde_json::value::Value::Bool;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SiteWithJs {
    pub url: String,
    pub json_key: String,
}

pub fn sites_with_js_enabled() -> Vec<SiteWithJs> {
    let preferences_json = std::fs::read_to_string(chrome::preferences()).unwrap();

    sites_on_javascript_safelist(&preferences_json)
}

fn sites_on_javascript_safelist(input: &str) -> Vec<SiteWithJs> {
    let v: Value = serde_json::from_str(input).unwrap();

    per_site_javascript_exceptions(&v)
        .into_iter()
        .filter(|(_k, v)| is_on_javascript_safelist(v))
        .filter_map(|(key, _config)| parse_site_from_key(key))
        .collect()
}

fn per_site_javascript_exceptions(v: &Value) -> Map<String, Value> {
    v["profile"]["content_settings"]["exceptions"]["javascript"]
        .as_object()
        .unwrap()
        .clone()
}

fn is_on_javascript_safelist(v: &Value) -> bool {
    let safelisted = v
        .as_object()
        .unwrap_or(&Map::new())
        .get("setting")
        .unwrap_or(&Bool(false))
        .as_u64()
        .unwrap();

    safelisted == 1
}

fn parse_site_from_key(json_key: String) -> Option<SiteWithJs> {
    let url = Regex::new(r"(?P<address>.*):\d+,.*")
        .ok()?
        .captures(&json_key)?
        .name("address")?
        .as_str()
        .to_owned();

    Some(SiteWithJs { url, json_key })
}

#[cfg(test)]
mod test {
    use crate::preferences::{sites_on_javascript_safelist, SiteWithJs};

    #[test]
    fn reads_js_enabled_sites() {
        let example = r#"
        {
            "profile": { "content_settings": { "exceptions": { "javascript": {
                "https://www.google.com:443,*": {
                    "expiration": "0",
                    "last_modified": "16188888000000",
                    "model": 0,
                    "setting": 1
                }
            }}}}
        }"#;

        let output = sites_on_javascript_safelist(example);

        assert_eq!(output.len(), 1);
        assert_eq!(
            output[0],
            SiteWithJs {
                url: "https://www.google.com".to_owned(),
                json_key: "https://www.google.com:443,*".to_owned()
            }
        )
    }
}
