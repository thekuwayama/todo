#[macro_use]
extern crate clap;

mod add;
mod list;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

const FILE_NAME: &str = ".todo_history";

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("add").arg(Arg::with_name("task")));
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", _) => {
            let f = File::open(log_file_path(FILE_NAME)).unwrap();
            let mut reader = BufReader::new(f);
            let res = list::list(&mut reader).unwrap_or(String::from("list error")); // TODO: error message
            println!("{}", res);
            ()
        }
        ("add", Some(s)) => {
            let f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path(FILE_NAME))
                .unwrap();
            let mut writer = BufWriter::new(f);
            add::add(&mut writer, s.value_of("task").unwrap()).unwrap();
            ()
        }
        ("add", None) => (), // TODO: print help
        _ => (),             // TODO: print help
    };
}

fn log_file_path(file_name: &str) -> String {
    match env::var("HOME") {
        Ok(val) => String::from(format!("{}/{}", val, file_name)),
        Err(_) => String::from(format!("./{}", file_name)),
    }
}
