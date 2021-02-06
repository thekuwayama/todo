use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

pub fn record<R: BufRead>(reader: &mut R, i: u32, t: f32) -> Result<String, Box<dyn error::Error>> {
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
            let s = caps.get(2).map_or("", |m| m.as_str());

            w.push_str(format!("{} {} ({:.1})\n", c, s, t).as_str());
        } else {
            w.push_str(format!("{}\n", l).as_str());
        }

        index += 1;
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
