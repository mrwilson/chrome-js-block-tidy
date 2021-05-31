use regex::Regex;
use serde_json::value::Value::Bool;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
pub struct SiteWithJavascriptEnabled {
    pub url: String,
}

pub fn sites_with_js_enabled() -> Vec<SiteWithJavascriptEnabled> {
    let home = std::env::var("HOME").unwrap();

    let preferences_json = std::fs::read_to_string(
        home + "/Library/Application Support/Google/Chrome/Default/Preferences",
    )
    .unwrap();

    sites_on_javascript_safelist(&preferences_json)
}

fn sites_on_javascript_safelist(input: &str) -> Vec<SiteWithJavascriptEnabled> {
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

fn parse_site_from_key(key: String) -> Option<SiteWithJavascriptEnabled> {
    let url = Regex::new(r"(?P<address>.*):\d+,.*")
        .ok()?
        .captures(&key)?
        .name("address")?
        .as_str()
        .to_owned();

    Some(SiteWithJavascriptEnabled { url })
}

#[cfg(test)]
mod test {
    use crate::preferences::{sites_on_javascript_safelist, SiteWithJavascriptEnabled};

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
            SiteWithJavascriptEnabled {
                url: "https://www.google.com".to_owned()
            }
        )
    }
}
