use std::error;
use std::io::BufRead;

use crate::format::Todo;

pub fn list<R: BufRead>(
    reader: &mut R,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let todo = Todo::deserialize(l.as_str())?;
        w.push_str(todo.list_string(index).as_str());

        index += 1;
        l.clear();
    }

    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_list() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            list(&mut reader).unwrap(),
            "\u{2611} 000: first\n\
             \u{2611} 001: second (2.0)\n\
             \u{2610} 002: third\n\
             \u{2610} 003: fourth (4.0)\n"
        );
    }
}
