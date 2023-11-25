use std::error;
use std::io::BufRead;

use crate::format::Todo;

pub(crate) fn sort<R: BufRead>(
    reader: &mut R,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut todo = String::new();
    let mut done = String::new();

    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let task = Todo::deserialize(l.as_str())?;
        if task.done {
            done.push_str(l.as_str());
        } else {
            todo.push_str(l.as_str());
        }

        l.clear();
    }

    Ok(done + todo.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_sort() {
        let mut reader = BufReader::new(
            "[ ] first ()\n\
             [x] second ()\n\
             [ ] third ()\n\
             [x] fourth ()\n"
                .as_bytes(),
        );
        assert_eq!(
            sort(&mut reader).unwrap(),
            "[x] second ()\n\
             [x] fourth ()\n\
             [ ] first ()\n\
             [ ] third ()\n"
        );
    }
}
