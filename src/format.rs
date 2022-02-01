use std::error;
use std::io::{Error, ErrorKind};

use nom::character::complete::{anychar, char, one_of};
use nom::combinator::opt;
use nom::multi::many_till;
use nom::number::complete::float;
use nom::IResult;

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";

pub(crate) struct Todo {
    pub done: bool,
    pub task: String,
    pub time: Option<f32>,
}

fn done(s: &str) -> IResult<&str, bool> {
    let (s, _) = char('[')(s)?;
    let (s, done) = one_of("x ")(s)?;
    let (s, _) = char(']')(s)?;

    Ok((s, done == 'x'))
}

fn time(s: &str) -> IResult<&str, Option<f32>> {
    let (s, _) = char('(')(s)?;
    let (s, time) = opt(float)(s)?;
    let (s, _) = char(')')(s)?;

    Ok((s, time))
}

fn todo(s: &str) -> IResult<&str, Todo> {
    let (s, done) = done(s)?;
    let (s, (task, time)) = many_till(anychar, time)(s)?;
    println!("{}", done);
    println!("{:?}", task);
    println!("{:?}", time);
    let todo = Todo {
        done,
        task: task.iter().collect::<String>().trim().to_string(),
        time,
    };

    Ok((s, todo))
}

impl Todo {
    pub fn serialize(&self) -> String {
        format!(
            "[{}] {} ({})\n",
            if self.done { 'x' } else { ' ' },
            self.task,
            self.time.map_or("".to_string(), |f| format!("{:.1}", f)),
        )
    }

    pub fn deserialize(s: &str) -> Result<Todo, Box<dyn error::Error + Send + Sync + 'static>> {
        let (s, todo) = match todo(s) {
            Ok((s, todo)) => (s, todo),
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::InvalidInput,
                    "failed to parse",
                )));
            }
        };
        if !s.is_empty() && s != "\n" {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "failed to parse",
            )));
        }

        Ok(todo)
    }

    pub fn list_string(&self, index: u32) -> String {
        if self.done && self.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                DONE,
                index,
                self.task,
                self.time.unwrap_or(0.0)
            )
        } else if self.done {
            format!("{} {:03}: {}\n", DONE, index, self.task)
        } else if !self.done && self.time.is_some() {
            format!(
                "{} {:03}: {} ({:.1})\n",
                TODO,
                index,
                self.task,
                self.time.unwrap_or(0.0)
            )
        } else {
            format!("{} {:03}: {}\n", TODO, index, self.task)
        }
    }

    pub fn report_string(&self) -> String {
        if self.done && self.time.is_some() {
            format!("- {} ({:.1}h)\n", self.task, self.time.unwrap_or(0.0))
        } else if self.done {
            format!("- {}\n", self.task)
        } else if self.time.is_some() {
            format!("- {} ({:.1}h)\n", self.task, self.time.unwrap_or(0.0))
        } else {
            format!("- {}\n", self.task)
        }
    }
}
