#[macro_use]
extern crate clap;

mod add;
mod list;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("add").arg(Arg::with_name("task")));
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", _) => {
            let f = File::open("/Users/thekuwayama/.todo").unwrap(); // TODO: home dir
            let mut reader = BufReader::new(f);
            let res = list::list(&mut reader).unwrap_or(String::from("list error"));
            println!("{}", res);
            ()
        }
        ("add", Some(s)) => {
            let f = OpenOptions::new()
                .append(true)
                .open("/Users/thekuwayama/.todo")
                .unwrap();
            let mut writer = BufWriter::new(f);
            add::add(&mut writer, s.value_of("task").unwrap()).unwrap();
            ()
        }
        ("add", None) => (),
        _ => (), // TODO: print help
    };
}
