use std::io::{BufRead, Error, ErrorKind};

pub fn delete<R: BufRead>(reader: &mut R, i: u32) -> Result<String, Error> {
    let mut w = String::new();

    let mut index = 1;
    for line in reader.lines() {
        let l = line?;
        if index != i {
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
    fn test_delete() {
        let mut reader = BufReader::new(
            "[ ] first ()\n\
             [ ] second ()\n\
             [ ] third ()\n"
                .as_bytes(),
        );
        assert_eq!(
            delete(&mut reader, 2).unwrap(),
            "[ ] first ()\n\
             [ ] third ()\n"
        );
    }
}
