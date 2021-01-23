use regex::Regex;

pub fn re() -> Regex {
    Regex::new(r"^(\[.\]) (.+) \(((\d+\.\d+)?)\)$").unwrap()
}
