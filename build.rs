use std::error;

use clap_complete::{generate_to, shells};

include!("src/cli.rs");

const COMPLETION_FILENAME: &str = "todo-completion";

fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let mut app = build();
    let _ = generate_to(shells::Bash, &mut app, COMPLETION_FILENAME, ".")?;

    Ok(())
}
