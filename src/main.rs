#[macro_use]
extern crate clap;

mod add;
mod r#continue;
mod delete;
mod done;
mod list;
mod record;
mod report;
mod swap;
mod undone;
mod unrecord;
mod utils;

use chrono::offset::Local;
use clap::{App, Arg, SubCommand};
use std::env;
use std::fs::{remove_file, rename, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

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
                .arg(Arg::with_name("comment").value_name("COMMENT"))
                .arg(
                    Arg::with_name("date")
                        .short("d")
                        .long("date")
                        .takes_value(true)
                        .value_name("DATE"),
                ),
        )
        .subcommand(SubCommand::with_name("continue").about("continue todo list"))
        .subcommand(SubCommand::with_name("uncontinue").about("uncontinue todo list"));

    let fp = log_file_path();
    let bp = format!("{}.backup", fp);
    let r = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(fp.clone())
        .expect(format!("failed to open the file {}", fp).as_str());
    let mut reader = BufReader::new(r);
    match app.clone().get_matches().subcommand() {
        ("list", _) => {
            let result = list::list(&mut reader).unwrap_or_else(|e| {
                eprintln!("failed to show todo list: {}", e);
                process::exit(1);
            });
            println!("{}", result);
            ()
        }
        ("clear", _) => {
            let _ = remove_file(fp.clone());
            ()
        }
        ("add", Some(s)) => {
            let result = add::add(s.value_of("task").unwrap());
            let mut writer = OpenOptions::new()
                .create(true)
                .append(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to add a task: {}", e);
                process::exit(1);
            });
            ()
        }
        ("delete", Some(i)) => {
            let result = delete::delete(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to delete a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to delete a task: {}", e);
                process::exit(1);
            });
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| {
                    eprintln!("failed to delete a task: {}", e);
                    process::exit(1);
                });
            ()
        }
        ("done", Some(i)) => {
            let result = done::done(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to done a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to done a task: {}", e);
                process::exit(1);
            });
            ()
        }
        ("undone", Some(i)) => {
            let result = undone::undone(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to undone a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to undone a task: {}", e);
                process::exit(1);
            });
            ()
        }
        ("record", Some(it)) => {
            let result = record::record(
                &mut reader,
                it.value_of("index").unwrap().parse::<u32>().unwrap(),
                it.value_of("time").unwrap().parse::<f32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to record time: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to record time: {}", e);
                process::exit(1);
            });
            ()
        }
        ("unrecord", Some(i)) => {
            let result = unrecord::unrecord(
                &mut reader,
                i.value_of("index").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to unrecord time: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to unrecord time: {}", e);
                process::exit(1);
            });
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| {
                    eprintln!("failed to unrecord time: {}", e);
                    process::exit(1);
                });
            ()
        }
        ("swap", Some(ii)) => {
            let result = swap::swap(
                &mut reader,
                ii.value_of("index1").unwrap().parse::<u32>().unwrap(),
                ii.value_of("index2").unwrap().parse::<u32>().unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to swap tasks: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to swap tasks: {}", e);
                process::exit(1);
            });
            ()
        }
        ("report", Some(cd)) => {
            let result = report::report(
                &mut reader,
                cd.value_of("comment").unwrap_or(""),
                cd.value_of("date")
                    .unwrap_or(Local::today().format("%Y/%m/%d").to_string().as_str()),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to report today's achievements: {}", e);
                process::exit(1);
            });
            println!("{}", result);
        }
        ("continue", _) => {
            let result = r#continue::r#continue(&mut reader).unwrap_or_else(|e| {
                eprintln!("failed to continue todo list: {}", e);
                process::exit(1);
            });
            rename(fp.clone(), bp).unwrap_or_else(|e| {
                eprintln!("failed to rename the file: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .create(true)
                .write(true)
                .open(fp.clone())
                .expect(format!("failed to open the file {}", fp).as_str());
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to continue todo list: {}", e);
                process::exit(1);
            });
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| {
                    eprintln!("failed to continue todo list: {}", e);
                    process::exit(1);
                });
            ()
        }
        ("uncontinue", _) => {
            if Path::new(bp.as_str()).exists() {
                rename(bp, fp.clone()).unwrap_or_else(|e| {
                    eprintln!("failed to rename the file {}", e);
                    process::exit(1);
                });
            }
            ()
        }
        _ => {
            let _ = app.to_owned().print_help();
            println!("")
        }
    };
}
