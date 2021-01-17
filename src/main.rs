#[macro_use]
extern crate clap;

mod list;

use clap::{App, SubCommand};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("list"));
    let matches = app.get_matches();

    let f = File::open("/Users/thekuwayama/.todo").unwrap(); // TODO: home dir
    let mut reader = BufReader::new(f);

    let result = match matches.subcommand() {
        ("list", _) => list::list(&mut reader).map_or_else(|e| e.to_string(), |v| v),
        _ => String::from("help"), // TODO: print help
    };
    println!("{}", result);
}
