use anyhow::{Context as _, Result};
use clap::Parser;
use std::{
    io::{self, BufRead, IsTerminal},
    path::PathBuf,
};

mod todo_item;
use crate::todo_item::TodoItem;

/// A todo list generator
#[derive(Debug, clap::Parser)]
#[command(version, about)]
struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text, value_name = "FORMAT")]
    output: OutputFormat,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Markdown,
    Text,
}

// TODO: This is a test
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use clap to parse command line arguments
    // For the output format
    // i.e. todors --output markdown
    // or todors --output json
    let args = Cli::parse();

    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Ok(());
    }

    let output = args.output;

    let mut todos = Vec::new();

    for path in stdin.lock().lines() {
        let path = path.expect("Error reading path");

        if let Ok(file) = std::fs::read_to_string(&path) {
            let mut new_items = todo_items_from_file(&file, &path)?;
            todos.append(&mut new_items)
        } else {
            continue;
        }
    }

    for todo in todos {
        use OutputFormat::*;
        let printable = match output {
            Text => todo.to_string(),
            Markdown => todo.to_markdown(),
        };
        println!("{}", printable);
    }
    Ok(())
}

fn todo_items_from_file(file: &str, path: &str) -> Result<Vec<TodoItem>> {
    file.lines()
        .enumerate()
        .filter_map(|(i, l)| {
            TodoItem::maybe_from_line(l).map(|mut builder| {
                builder
                    .path(PathBuf::from(path))
                    .line_number(i)
                    .build()
                    .context("Failed to build todoitem")
            })
        })
        .collect::<Result<Vec<_>>>()
}
