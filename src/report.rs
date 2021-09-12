use std::error;
use std::io::{BufRead, Error, ErrorKind};

use crate::utils;

pub fn report<R: BufRead>(
    reader: &mut R,
    comment: &str,
    date: &str,
) -> Result<String, Box<dyn error::Error + Send + Sync + 'static>> {
    let re = utils::re();
    let mut doings = String::new();
    let mut dones = String::new();
    let mut todos = String::new();
    let mut elapsed = 0.0;

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
            ("[x]", s, "") => dones.push_str(format!("- {}\n", s).as_str()),
            ("[x]", s, t) => {
                dones.push_str(format!("- {} ({}h)\n", s, t).as_str());
                elapsed += t.parse::<f32>()?;
            }
            ("[ ]", s, "") => todos.push_str(format!("- {}\n", s).as_str()),
            ("[ ]", s, t) => {
                doings.push_str(format!("- {} ({}h)\n", s, t).as_str());
                elapsed += t.parse::<f32>()?;
            }
            _ => (),
        };

        l.clear();
    }

    Ok(format!(
        "## {} ({:.1}h)\n\
         ### 進行中のタスク\n\
         {}\n\
         ### 完了済みのタスク\n\
         {}\n\
         ### その他、今週対応予定のタスク (金曜日は来週対応予定のタスク)\n\
         {}\n\
         ### メモ、ぼやき\n\
         {}\n",
        date, elapsed, doings, dones, todos, comment
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_report() {
        let mut reader = BufReader::new(
            "[x] first ()\n\
             [x] second (2.0)\n\
             [ ] third ()\n\
             [ ] fourth (4.0)\n"
                .as_bytes(),
        );
        assert_eq!(
            report(&mut reader, "test", "2020/01/22").unwrap(),
            "## 2020/01/22 (6.0h)\n\
             ### 進行中のタスク\n\
             - fourth (4.0h)\n\
             \n\
             ### 完了済みのタスク\n\
             - first\n\
             - second (2.0h)\n\
             \n\
             ### その他、今週対応予定のタスク (金曜日は来週対応予定のタスク)\n\
             - third\n\
             \n\
             ### メモ、ぼやき\n\
             test\n",
        );
    }
}
