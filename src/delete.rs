use std::error;
use std::io::{BufRead, Error, ErrorKind};

pub fn delete<R: BufRead>(
    reader: &mut R,
    i: u32,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        if index != i {
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
    fn test_delete() {
        let mut reader = BufReader::new(
            "[ ] first ()\n\
             [ ] second ()\n\
             [ ] third ()\n"
                .as_bytes(),
        );
        assert_eq!(
            delete(&mut reader, 1).unwrap(),
            "[ ] first ()\n\
             [ ] third ()\n"
        );
    }
}
