#[macro_use]
extern crate clap;

mod add;
mod done;
mod list;
mod undone;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

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
                .about("add the task")
                .arg(Arg::with_name("task").required(true)),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("done the task")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("undone")
                .about("undone the task")
                .arg(Arg::with_name("index").required(true)),
        );

    let path = log_file_path();
    let r = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path.clone())
        .expect(format!("failed to file open {}", path).as_str());
    let mut reader = BufReader::new(r);
    match app.clone().get_matches().subcommand() {
        ("list", _) => {
            let res = list::list(&mut reader).unwrap_or(String::from("failed to list"));
            println!("{}", res);
            ()
        }
        ("add", Some(s)) => {
            let err_msg = "failed to add a task";
            let result = add::add(s.value_of("task").unwrap()).expect(err_msg);
            let mut writer = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer.write(result.as_bytes()).expect(err_msg);
            writer.flush().expect(err_msg);
            ()
        }
        ("done", Some(i)) => {
            let err_msg = "failed to done a task";
            let result = done::done(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .expect(err_msg);
            let mut writer = OpenOptions::new()
                .create(true)
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer.write(result.as_bytes()).expect(err_msg);
            writer.flush().expect(err_msg);
            ()
        }
        ("undone", Some(i)) => {
            let err_msg = "failed to undone a task";
            let result = undone::undone(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .expect(err_msg);
            let mut writer = OpenOptions::new()
                .create(true)
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer.write(result.as_bytes()).expect(err_msg);
            writer.flush().expect(err_msg);
            ()
        }
        _ => {
            let _ = app.to_owned().print_help();
            println!("")
        }
    };
}
