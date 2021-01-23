use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

pub fn r#continue<R: BufRead>(reader: &mut R) -> Result<String, Error> {
    let re = utils::re();
    let mut w = String::new();

    for line in reader.lines() {
        let l = line?;
        let caps = re
            .captures(l.as_str())
            .ok_or(Error::new(ErrorKind::InvalidInput, "format error"))?;
        match (
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
        ) {
            ("[x]", _) => continue,
            ("[ ]", s) => w.push_str(format!("[ ] {} ()\n", s).as_str()),
            _ => (),
        };
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
            "[x] first ()\n[x] second (2.0)\n[ ] third ()\n[ ] fourth (4.0)\n".as_bytes(),
        );
        assert!(r#continue(&mut reader).is_ok());
        reader = BufReader::new(
            "[x] first ()\n[x] second (2.0)\n[ ] third ()\n[ ] fourth (4.0)\n".as_bytes(),
        );
        assert_eq!(
            r#continue(&mut reader).unwrap(),
            "[ ] third ()\n\
             [ ] fourth ()\n"
        );
    }
}
