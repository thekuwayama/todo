use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::format::Todo;

pub(crate) fn show<R: BufRead>(
    reader: &mut R,
    i: u32,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        if index == i {
            let todo = Todo::deserialize(l.as_str())?;
            w.push_str(todo.task.as_str());
            return Ok(w);
        }

        index += 1;
        l.clear();
    }

    Err(Box::new(Error::new(
        ErrorKind::InvalidInput,
        "invalid index",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_show() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(show(&mut reader, 1).unwrap(), "second",);
    }
}
