use std::collections::HashMap;
use std::error;
use std::io::{BufRead, Error, ErrorKind};

use once_cell::sync::Lazy;

use crate::utils;

#[derive(Debug, Hash, Eq, PartialEq)]
enum ReportKey {
    Doing,
    Done,
    Todo,
    Comment,
}

static JA: Lazy<HashMap<ReportKey, &str>> = Lazy::new(|| {
    HashMap::from([
        (ReportKey::Doing, "進行中のタスク"),
        (ReportKey::Done, "完了済みのタスク"),
        (
            ReportKey::Todo,
            "その他、今週対応予定のタスク（金曜日は来週対応予定のタスク）",
        ),
        (ReportKey::Comment, "メモ、ぼやき"),
    ])
});

static EN: Lazy<HashMap<ReportKey, &str>> = Lazy::new(|| {
    HashMap::from([
        (ReportKey::Doing, "Doing tasks"),
        (ReportKey::Done, "Done tasks"),
        (
            ReportKey::Todo,
            "Todo tasks in this week (On Friday, next week scheduled tasks)",
        ),
        (ReportKey::Comment, "Memo & Comments"),
    ])
});

static ZH: Lazy<HashMap<ReportKey, &str>> = Lazy::new(|| {
    HashMap::from([
        (ReportKey::Doing, "进行中的任务"),
        (ReportKey::Done, "已完成的任务"),
        (ReportKey::Todo, "本周计划支持的其他任务（下周周五支持）"),
        (ReportKey::Comment, "备忘"),
    ])
});

pub fn report<R: BufRead>(
    reader: &mut R,
    comment: &str,
    title: &str,
    lang: &str,
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

    let desc = match lang {
        "ja" => &JA,
        "en" => &EN,
        "zh" => &ZH,
        &_ => &JA,
    };
    Ok(do_report(
        title, elapsed, &doings, &dones, &todos, comment, desc,
    ))
}

fn do_report(
    title: &str,
    elapsed: f32,
    doings: &str,
    dones: &str,
    todos: &str,
    comment: &str,
    desc: &HashMap<ReportKey, &str>,
) -> String {
    format!(
        "## {} ({:.1}h)\n\
         ### {}\n\
         {}\n\
         ### {}\n\
         {}\n\
         ### {}\n\
         {}\n\
         ### {}\n\
         {}\n",
        title,
        elapsed,
        desc.get(&ReportKey::Doing).unwrap(),
        doings,
        desc.get(&ReportKey::Done).unwrap(),
        dones,
        desc.get(&ReportKey::Todo).unwrap(),
        todos,
        desc.get(&ReportKey::Comment).unwrap(),
        comment
    )
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
            report(&mut reader, "test", "2020/01/22", "ja").unwrap(),
            "## 2020/01/22 (6.0h)\n\
             ### 進行中のタスク\n\
             - fourth (4.0h)\n\
             \n\
             ### 完了済みのタスク\n\
             - first\n\
             - second (2.0h)\n\
             \n\
             ### その他、今週対応予定のタスク（金曜日は来週対応予定のタスク）\n\
             - third\n\
             \n\
             ### メモ、ぼやき\n\
             test\n",
        );
    }
}
