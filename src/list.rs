use std::error;
use std::io::BufRead;

use crate::format::Todo;
use crate::string::StringExt;

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";

struct List(Todo);
impl List {
    fn serialize(&self, index: u32) -> String {
        if self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                DONE,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
            .cyan()
        } else if self.0.done {
            format!("{} {:03}: {}\n", DONE, index, self.0.task).cyan()
        } else if !self.0.done && self.0.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                TODO,
                index,
                self.0.task,
                self.0.time.unwrap_or(0.0)
            )
        } else {
            format!("{} {:03}: {}\n", TODO, index, self.0.task)
        }
        .bold()
    }
}

pub(crate) fn list<R: BufRead>(
    reader: &mut R,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let mut w = String::new();

    let mut index = 0;
    let mut l = String::new();
    while reader.read_line(&mut l)? > 0 {
        let todo = List(Todo::deserialize(l.as_str())?);
        w.push_str(todo.serialize(index).as_str());

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
            "\u{1b}[1m\u{1b}[36m\u{2611} 000: first\n\u{1b}[0m\u{1b}[0m\
             \u{1b}[1m\u{1b}[36m\u{2611} 001: second (2.0)\n\u{1b}[0m\u{1b}[0m\
             \u{1b}[1m\u{2610} 002: third\n\u{1b}[0m\
             \u{1b}[1m\u{2610} 003: fourth (4.0)\n\u{1b}[0m"
        );
    }
}
