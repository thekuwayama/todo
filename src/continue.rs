use std::error;
use std::io::BufRead;

use crate::format::Todo;

pub fn r#continue<R: BufRead>(
    reader: &mut R,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let mut todo = Todo::deserialize(l.as_str())?;
        if !todo.done {
            todo.time = None;
            w.push_str(todo.serialize().as_str());
        }

        l.clear();
    }

    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_continue() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            r#continue(&mut reader).unwrap(),
            "[ ] third ()\n\
             [ ] fourth ()\n"
        );
    }
}
