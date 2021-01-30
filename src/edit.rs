use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

pub fn edit<R: BufRead>(reader: &mut R, i: u32, s: &str) -> Result<String, Error> {
    let re = utils::re();
    let mut w = String::new();

    let mut index = 0;
    for line in reader.lines() {
        let l = line?;
        let caps = re
            .captures(l.as_str())
            .ok_or(Error::new(ErrorKind::InvalidInput, "format error"))?;
        if index == i {
            let c = caps.get(1).map_or("", |m| m.as_str());
            let t = caps.get(3).map_or("", |m| m.as_str());

            w.push_str(format!("{} {} ({})\n", c, s, t).as_str());
        } else {
            w.push_str(format!("{}\n", l).as_str());
        }

        index += 1;
    }

    if index <= i {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid index"));
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
