#[macro_use]
extern crate clap;

mod add;
mod r#continue;
mod delete;
mod done;
mod edit;
mod list;
mod record;
mod report;
mod swap;
mod undone;
mod unrecord;
mod utils;

use chrono::offset::Local;
use clap::{arg, crate_name, App, AppSettings};
use std::env;
use std::fs::{remove_file, rename, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

const FILE_NAME: &str = ".todo";

fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => [&val, FILE_NAME].join("/"),
        Err(_) => format!("./{}", FILE_NAME),
    }
}

fn build_cli() -> App<'static> {
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
                .arg(arg!(<TITLE>).required(false)),
        )
        .subcommand(App::new("continue").about("continue todo list"))
        .subcommand(App::new("uncontinue").about("uncontinue todo list"))
}

fn main() {
    let fp = log_file_path();
    let bp = format!("{}.backup", fp);
    let r = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&fp)
        .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
    let mut reader = BufReader::new(r);
    match build_cli().get_matches().subcommand().unwrap() {
        ("list", _) => {
            let result = list::list(&mut reader).unwrap_or_else(|e| {
                eprintln!("failed to show todo list: {}", e);
                process::exit(1);
            });
            println!("{}", result);
        }
        ("clear", _) => {
            let _ = remove_file(&fp);
        }
        ("add", s) => {
            let result = add::add(s.value_of("TASK").unwrap());
            let mut writer = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to add a task: {}", e);
                process::exit(1);
            });
        }
        ("delete", i) => {
            let result = delete::delete(
                &mut reader,
                i.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to delete a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
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
        }
        ("edit", it) => {
            let result = edit::edit(
                &mut reader,
                it.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
                it.value_of("TASK").unwrap(),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to edit task description: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to edit task description: {}", e);
                process::exit(1);
            });
            writer
                .set_len(result.as_bytes().len() as u64)
                .unwrap_or_else(|e| {
                    eprintln!("failed to edit task description: {}", e);
                    process::exit(1);
                });
        }
        ("done", i) => {
            let result = done::done(
                &mut reader,
                i.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to done a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to done a task: {}", e);
                process::exit(1);
            });
        }
        ("undone", i) => {
            let result = undone::undone(
                &mut reader,
                i.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to undone a task: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to undone a task: {}", e);
                process::exit(1);
            });
        }
        ("record", it) => {
            let result = record::record(
                &mut reader,
                it.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
                it.value_of("TIME")
                    .unwrap()
                    .parse::<f32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <TIME> should be float");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to record time: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to record time: {}", e);
                process::exit(1);
            });
        }
        ("unrecord", i) => {
            let result = unrecord::unrecord(
                &mut reader,
                i.value_of("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to unrecord time: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
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
        }
        ("swap", ii) => {
            let result = swap::swap(
                &mut reader,
                ii.value_of("INDEX1")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX1> should be integer");
                        process::exit(1);
                    }),
                ii.value_of("INDEX2")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX2> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to swap tasks: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to swap tasks: {}", e);
                process::exit(1);
            });
        }
        ("report", cd) => {
            let date = Local::today().format("%Y/%m/%d").to_string();
            let result = report::report(
                &mut reader,
                cd.value_of("COMMENT").unwrap_or(""),
                cd.value_of("TITLE").unwrap_or(&date),
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
            rename(&fp, bp).unwrap_or_else(|e| {
                eprintln!("failed to rename the file: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
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
        }
        ("uncontinue", _) => {
            if Path::new(bp.as_str()).exists() {
                rename(bp, &fp).unwrap_or_else(|e| {
                    eprintln!("failed to rename the file {}", e);
                    process::exit(1);
                });
            }
        }
        _ => unreachable!(),
    };
}
