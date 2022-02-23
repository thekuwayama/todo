use std::fmt::Display;
use std::str::FromStr;

use clap::{arg, crate_description, crate_name, crate_version, ArgEnum, Command, PossibleValue};

#[derive(ArgEnum, Clone, Copy)]
pub enum Language {
    Ja,
    En,
    Zh,
}

impl FromStr for Language {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl Language {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Self::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

pub fn build() -> Command<'static> {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("list").about("show todo list"))
        .subcommand(Command::new("clear").about("clear todo list"))
        .subcommand(
            Command::new("add")
                .about("add the task")
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new("delete")
                .about("delete the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new("edit")
                .about("edit the task description")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new("done")
                .about("done the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new("undone")
                .about("undone the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new("record")
                .about("record elapsed time")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TIME>).required(true)),
        )
        .subcommand(
            Command::new("unrecord")
                .about("unrecord elapsed time")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new("swap")
                .about("swap two tasks")
                .arg(arg!(<INDEX1>).required(true))
                .arg(arg!(<INDEX2>).required(true)),
        )
        .subcommand(
            Command::new("report")
                .about("report today's achievements")
                .arg(arg!(<COMMENT>).required(false))
                .arg(arg!(<TITLE>).required(false))
                .arg(
                    arg!(<LANG>)
                        .long("lang")
                        .short('l')
                        .default_value("ja")
                        .takes_value(true)
                        .possible_values(Language::possible_values())
                        .required(false),
                ),
        )
        .subcommand(Command::new("continue").about("continue todo list"))
        .subcommand(Command::new("uncontinue").about("uncontinue todo list"))
}
