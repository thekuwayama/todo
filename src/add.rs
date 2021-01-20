use std::io::Error;

pub fn add(s: &str) -> Result<String, Error> {
    Ok(format!("[ ] {} ()\n", s).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add("test").is_ok());
        assert_eq!(add("test").unwrap(), "[ ] test ()\n".to_string());
    }
}
