use std::process;

use clap_complete::{generate_to, shells};

include!("src/cli.rs");

const COMPLETION_FILE_NAME: &str = "todo-completion";

fn main() {
    let mut app = build();
    let _ = generate_to(shells::Bash, &mut app, COMPLETION_FILE_NAME, ".").unwrap_or_else(|e| {
        eprintln!("failed to generate {}.bash: {}", COMPLETION_FILE_NAME, e);
        process::exit(1);
    });
}
