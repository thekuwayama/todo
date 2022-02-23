mod add;
mod cli;
mod r#continue;
mod delete;
mod done;
mod edit;
mod format;
mod list;
mod record;
mod report;
mod swap;
mod undone;
mod unrecord;

use std::env;
use std::fs::{remove_file, rename, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;
use std::str::FromStr;

use chrono::offset::Local;

use crate::cli::Language;

const FILE_NAME: &str = ".todo";

fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => [&val, FILE_NAME].join("/"),
        Err(_) => format!("./{}", FILE_NAME),
    }
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
    match cli::build().get_matches().subcommand().unwrap() {
        (cli::LIST, _) => {
            let result = list::list(&mut reader).unwrap_or_else(|e| {
                eprintln!("failed to show todo list: {}", e);
                process::exit(1);
            });
            println!("{}", result);
        }
        (cli::CLEAR, _) => {
            let _ = remove_file(&fp);
        }
        (cli::ADD, s) => {
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
        (cli::DELETE, i) => {
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
        (cli::EDIT, it) => {
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
        (cli::DONE, i) => {
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
        (cli::UNDONE, i) => {
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
        (cli::RECORD, it) => {
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
        (cli::UNRECORD, i) => {
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
        (cli::SWAP, ii) => {
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
        (cli::REPORT, cdl) => {
            let date = Local::today().format("%Y/%m/%d").to_string();
            let lang = Language::from_str(cdl.value_of("LANG").unwrap_or("ja")).unwrap();
            let result = report::report(
                &mut reader,
                cdl.value_of("COMMENT").unwrap_or(""),
                cdl.value_of("TITLE").unwrap_or(&date),
                &lang,
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to report today's achievements: {}", e);
                process::exit(1);
            });
            println!("{}", result);
        }
        (cli::CONTINUE, _) => {
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
        (cli::UNCONTINUE, _) => {
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
