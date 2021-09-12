pub fn add(s: &str) -> String {
    format!("[ ] {} ()\n", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("test"), "[ ] test ()\n".to_string());
    }
}
