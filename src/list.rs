use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";

pub fn list<R: BufRead>(
    reader: &mut R,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let re = utils::re();
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let caps = re
            .captures(l.as_str())
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "format error"))?;
        match (
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
            caps.get(3).map_or("", |m| m.as_str()),
        ) {
            ("[x]", s, "") => w.push_str(format!("{} {:03}: {}\n", DONE, index, s).as_str()),
            ("[x]", s, t) => w.push_str(format!("{} {:03}: {} ({})\n", DONE, index, s, t).as_str()),
            ("[ ]", s, "") => w.push_str(format!("{} {:03}: {}\n", TODO, index, s).as_str()),
            ("[ ]", s, t) => w.push_str(format!("{} {:03}: {} ({})\n", TODO, index, s, t).as_str()),
            _ => (),
        };

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
