use crate::format::Todo;

pub fn add(s: &str) -> String {
    Todo {
        done: false,
        task: s.to_string(),
        time: None,
    }
    .serialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("test"), "[ ] test ()\n".to_string());
    }
}
