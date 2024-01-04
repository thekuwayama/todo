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
mod show;
mod sort;
mod string;
mod swap;
mod undone;
mod unrecord;

use std::env;
use std::fs::{remove_file, rename, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

use time::{format_description, OffsetDateTime};

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
            let result = add::add(s.get_one::<String>("TASK").unwrap());
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
                i.get_one::<String>("INDEX")
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
                it.get_one::<String>("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
                it.get_one::<String>("TASK").unwrap(),
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
                i.get_one::<String>("INDEX")
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
                i.get_one::<String>("INDEX")
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
                it.get_one::<String>("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
                it.get_one::<String>("TIME")
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
                i.get_one::<String>("INDEX")
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
        (cli::SHOW, i) => {
            let result = show::show(
                &mut reader,
                i.get_one::<String>("INDEX")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX> should be integer");
                        process::exit(1);
                    }),
            )
            .unwrap_or_else(|e| {
                eprintln!("failed to show the task: {}", e);
                process::exit(1);
            });
            print!("{}", result);
        }
        (cli::SORT, _) => {
            let result = sort::sort(&mut reader).unwrap_or_else(|e| {
                eprintln!("failed to swap tasks: {}", e);
                process::exit(1);
            });
            let mut writer = OpenOptions::new()
                .write(true)
                .open(&fp)
                .unwrap_or_else(|_| panic!("failed to open the file {}", fp));
            writer.write_all(result.as_bytes()).unwrap_or_else(|e| {
                eprintln!("failed to sort tasks: {}", e);
                process::exit(1);
            });
        }
        (cli::SWAP, ii) => {
            let result = swap::swap(
                &mut reader,
                ii.get_one::<String>("INDEX1")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap_or_else(|_| {
                        eprintln!("failed, <INDEX1> should be integer");
                        process::exit(1);
                    }),
                ii.get_one::<String>("INDEX2")
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
            let date_format =
                format_description::parse("[year]/[month padding:zero]/[day padding:zero]")
                    .unwrap();
            let date = OffsetDateTime::now_local()
                .unwrap_or_else(|e| {
                    eprintln!("failed to get datetime: {}", e);
                    process::exit(1);
                })
                .format(&date_format)
                .unwrap_or_else(|e| {
                    eprintln!("failed to get datetime: {}", e);
                    process::exit(1);
                });
            let lang = cdl.get_one::<Language>("LANG").unwrap_or(&Language::Ja);
            let result = report::report(
                &mut reader,
                cdl.get_one::<String>("COMMENT").unwrap_or(&"".to_owned()),
                cdl.get_one::<String>("TITLE").unwrap_or(&date),
                lang,
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
