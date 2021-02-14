use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

pub fn done<R: BufRead>(
    reader: &mut R,
    i: u32,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let re = utils::re();
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let caps = re
            .captures(l.as_str())
            .ok_or(Error::new(ErrorKind::InvalidInput, "format error"))?;
        if index == i {
            let s = caps.get(2).map_or("", |m| m.as_str());
            let t = caps.get(3).map_or("", |m| m.as_str());

            w.push_str(format!("[x] {} ({})\n", s, t).as_str());
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
    fn test_done() {
        let mut reader = BufReader::new(
            "[ ] first ()\n\
             [ ] second ()\n"
                .as_bytes(),
        );
        assert_eq!(
            done(&mut reader, 0).unwrap(),
            "[x] first ()\n\
             [ ] second ()\n"
        );
    }
}
