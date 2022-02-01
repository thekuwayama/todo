use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::format::Todo;

pub fn edit<R: BufRead>(
    reader: &mut R,
    i: u32,
    s: &str,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        if index == i {
            let mut todo = Todo::deserialize(l.as_str())?;
            todo.task = s.to_string();
            w.push_str(todo.serialize().as_str());
        } else {
            w.push_str(l.as_str());
        }

        index += 1;
        l.clear();
    }

    if index <= i {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "invalid index",
        )));
    }

    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_edit() {
        let mut reader = BufReader::new(
            "[x] first (1.0)\n\
             [x] second (2.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            edit(&mut reader, 1, "edited").unwrap(),
            "[x] first (1.0)\n\
             [x] edited (2.0)\n"
        );
    }
}
