use clap::{arg, crate_description, crate_name, crate_version, App, AppSettings};

pub fn build_cli() -> App<'static> {
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
