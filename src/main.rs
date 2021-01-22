#[macro_use]
extern crate clap;
extern crate chrono;

mod add;
mod delete;
mod done;
mod list;
mod record;
mod report;
mod swap;
mod undone;
mod unrecord;

use chrono::offset::Local;
use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{remove_file, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

const FILE_NAME: &str = ".todo";

fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => format!("{}/{}", val, FILE_NAME).to_string(),
        Err(_) => format!("./{}", FILE_NAME).to_string(),
    }
}

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("list").about("show todo list"))
        .subcommand(SubCommand::with_name("clear").about("clear todo list"))
        .subcommand(
            SubCommand::with_name("add")
                .about("add the task")
                .arg(Arg::with_name("task").required(true)),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("delete the task")
                .arg(Arg::with_name("index").required(true)),
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
        )
        .subcommand(
            SubCommand::with_name("record")
                .about("record elapsed time")
                .args(&[
                    Arg::with_name("index").required(true),
                    Arg::with_name("time").required(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("unrecord")
                .about("unrecord elapsed time")
                .arg(Arg::with_name("index").required(true)),
        )
        .subcommand(
            SubCommand::with_name("swap")
                .about("swap two tasks")
                .args(&[
                    Arg::with_name("index1").required(true),
                    Arg::with_name("index2").required(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("report")
                .about("report today's achievements")
                .args(&[Arg::with_name("comment"), Arg::with_name("date")]),
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
            let result =
                list::list(&mut reader).unwrap_or_else(|e| panic!("failed to list: {}", e));
            println!("{}", result);
            ()
        }
        ("clear", _) => {
            let _ = remove_file(path.clone());
            ()
        }
        ("add", Some(s)) => {
            let result = add::add(s.value_of("task").unwrap())
                .unwrap_or_else(|e| panic!("failed to add a task: {}", e));
            let mut writer = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to add a task: {}", e));
            ()
        }
        ("delete", Some(i)) => {
            let result = delete::delete(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to delete a task: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to delete a task: {}", e));
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| panic!("failed to delete a task: {}", e));
            ()
        }
        ("done", Some(i)) => {
            let result = done::done(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to done a task: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to done a task: {}", e));
            ()
        }
        ("undone", Some(i)) => {
            let result = undone::undone(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to undone a task: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to undone a task: {}", e));
            ()
        }
        ("record", Some(it)) => {
            let result = record::record(
                &mut reader,
                it.value_of("index").unwrap().parse::<u32>().unwrap(),
                it.value_of("time").unwrap().parse::<f32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to record time: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to record time: {}", e));
            ()
        }
        ("unrecord", Some(i)) => {
            let result = unrecord::unrecord(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to unrecord time: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to unrecord time: {}", e));
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| panic!("failed to unrecord time: {}", e));
            ()
        }
        ("swap", Some(ii)) => {
            let result = swap::swap(
                &mut reader,
                ii.value_of("index1").unwrap().parse::<u32>().unwrap(),
                ii.value_of("index2").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| panic!("failed to swap tasks: {}", e));
            let mut writer = OpenOptions::new()
                .write(true)
                .open(path.clone())
                .expect(format!("failed to file open {}", path).as_str());
            writer
                .write_all(result.as_bytes())
                .unwrap_or_else(|e| panic!("failed to swap tasks: {}", e));
            ()
        }
        ("report", Some(cd)) => {
            let result = report::report(
                &mut reader,
                cd.value_of("comment").unwrap_or(""),
                cd.value_of("date")
                    .unwrap_or(Local::today().format("%Y/%m/%d").to_string().as_str()),
            )
            .unwrap_or_else(|e| panic!("failed to report today's achievements: {}", e));
            println!("{}", result);
        }
        _ => {
            let _ = app.to_owned().print_help();
            println!("")
        }
    };
}
