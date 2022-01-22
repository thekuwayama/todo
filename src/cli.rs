use std::fmt::Display;
use std::str::FromStr;

use clap::{
    arg, crate_description, crate_name, crate_version, App, AppSettings, ArgEnum, PossibleValue,
};

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

pub fn build() -> App<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("list").about("show todo list"))
        .subcommand(App::new("clear").about("clear todo list"))
        .subcommand(
            App::new("add")
                .about("add the task")
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            App::new("delete")
                .about("delete the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            App::new("edit")
                .about("edit the task description")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            App::new("done")
                .about("done the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            App::new("undone")
                .about("undone the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            App::new("record")
                .about("record elapsed time")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TIME>).required(true)),
        )
        .subcommand(
            App::new("unrecord")
                .about("unrecord elapsed time")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            App::new("swap")
                .about("swap two tasks")
                .arg(arg!(<INDEX1>).required(true))
                .arg(arg!(<INDEX2>).required(true)),
        )
        .subcommand(
            App::new("report")
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
        .subcommand(App::new("continue").about("continue todo list"))
        .subcommand(App::new("uncontinue").about("uncontinue todo list"))
}
