use regex::Regex;

pub(crate) fn re() -> Regex {
    Regex::new(r"^(\[.\]) (.*) \(((\d+\.\d+)?)\)\n$").unwrap()
}
