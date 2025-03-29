use std::error;
use std::io::{Error, ErrorKind};

use nom::character::complete::{anychar, char, one_of};
use nom::combinator::opt;
use nom::multi::many_till;
use nom::number::complete::float;
use nom::sequence::{delimited, terminated};
use nom::IResult;

const PARSE_ERROR: &str = "failed to parse";

pub(crate) struct Todo {
    pub done: bool,
    pub task: String,
    pub time: Option<f32>,
}

fn done(s: &str) -> IResult<&str, bool> {
    let (s, done) = delimited(char('['), one_of("x "), char(']'))(s)?;

    Ok((s, done == 'x'))
}

fn time(s: &str) -> IResult<&str, Option<f32>> {
    let (s, time) = delimited(char('('), opt(float), char(')'))(s)?;

    Ok((s, time))
}

fn todo(s: &str) -> IResult<&str, Todo> {
    let (s, done) = done(s)?;
    let (s, (task, time)) = many_till(anychar, terminated(time, char('\n')))(s)?;
    let mut task = task;
    if task[0] != ' ' || task.remove(0) != ' ' || task.pop() != Some(' ') {
        return Err(nom::Err::Error(nom::error::Error::new(
            PARSE_ERROR,
            nom::error::ErrorKind::Char,
        )));
    }
    let task = task.iter().collect();
    let todo = Todo { done, task, time };

    Ok((s, todo))
}

impl Todo {
    pub(crate) fn serialize(&self) -> String {
        format!(
            "[{}] {} ({})\n",
            if self.done { 'x' } else { ' ' },
            self.task,
            self.time.map_or("".to_string(), |f| format!("{:.1}", f)),
        )
    }

    pub(crate) fn deserialize(
        s: &str,
    ) -> Result<Todo, Box<dyn error::Error + Send + Sync + 'static>> {
        match todo(s) {
            Ok(("", todo)) => Ok(todo),
            _ => Err(Box::new(Error::new(ErrorKind::InvalidInput, PARSE_ERROR))),
        }
    }
}
