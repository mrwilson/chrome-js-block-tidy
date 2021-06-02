fn home() -> String {
    std::env::var("HOME").unwrap() + "/Library/Application Support/Google/Chrome/Default/"
}

pub fn preferences() -> String {
    home() + "Preferences"
}

pub fn history() -> String {
    home() + "History"
}
