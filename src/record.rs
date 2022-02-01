use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::format::Todo;

pub fn record<R: BufRead>(
    reader: &mut R,
    i: u32,
    t: f32,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        if index == i {
            let mut todo = Todo::deserialize(l.as_str())?;
            todo.time = Some(t);
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
    fn test_record() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second ()\n"
                .as_bytes(),
        );
        assert_eq!(
            record(&mut reader, 0, 0.5).unwrap(),
            "[x] first (0.5)\n\
             [x] second ()\n"
        );
    }
}
