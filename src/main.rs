#[macro_use]
extern crate clap;

mod add;
mod list;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

const FILE_NAME: &str = ".todo_history";

fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => String::from(format!("{}/{}", val, FILE_NAME)),
        Err(_) => String::from(format!("./{}", FILE_NAME)),
    }
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("list").about("show todo list"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add todo task")
                .arg(Arg::with_name("task").required(true)),
        );

    let path = log_file_path();
    match app.clone().get_matches().subcommand() {
        ("list", _) => {
            let f =
                File::open(path.clone()).expect(format!("failed to file open {}", path).as_str());
            let mut reader = BufReader::new(f);
            let res = list::list(&mut reader).unwrap_or(String::from("failed to list"));
            println!("{}", res);
            ()
        }
        ("add", Some(s)) => {
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            let mut writer = BufWriter::new(f);
            add::add(&mut writer, s.value_of("task").unwrap()).unwrap();
            ()
        }
        _ => {
            let _ = app.clone().print_help();
            println!("")
        }
    };
}
