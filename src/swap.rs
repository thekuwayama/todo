use std::cmp::{max, min};
use std::io::{BufRead, Error, ErrorKind};

pub fn swap<R: BufRead>(reader: &mut R, i1: u32, i2: u32) -> Result<String, Error> {
    let mut w = String::new();
    let mut buf = String::new();
    let mut tmp = String::new();
    let lh = min(i1, i2);
    let rh = max(i1, i2);

    let mut index = 1;
    for line in reader.lines() {
        let l = line?;
        if index < lh || index > rh {
            w.push_str(format!("{}\n", l).as_str());
        } else if index == lh {
            tmp = l;
        } else if index < rh {
            buf.push_str(format!("{}\n", l).as_str());
        } else if index == rh {
            w.push_str(format!("{}\n", l).as_str());
            w.push_str(buf.as_str());
            w.push_str(format!("{}\n", tmp).as_str());
        }

        index += 1;
    }

    if index <= lh || index <= rh {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid index"));
    }

    Ok(w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_swap() {
        let mut reader = BufReader::new("[x] first ()\n[x] second ()\n[x] third ()\n".as_bytes());
        assert!(swap(&mut reader, 1, 3).is_ok());
        reader = BufReader::new("[x] first ()\n[x] second ()\n[x] third ()\n".as_bytes());
        assert_eq!(
            swap(&mut reader, 1, 3).unwrap(),
            "[x] third ()\n\
             [x] second ()\n\
             [x] first ()\n"
        );
    }
}
