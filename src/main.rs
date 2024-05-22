use anyhow::{Context as _, Result};
use clap::Parser;
use itertools::Itertools;
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

    // Group output by a property
    #[arg(short, long, value_enum, value_name = "GROUP")]
    group_by: Option<GroupBy>,

    // Show todos with file name and line number or just the task
    #[arg(short, long, value_enum, value_name = "DETAIL", default_value_t = todo_item::DisplayDetail::FileAndLineNumber)]
    detail: todo_item::DisplayDetail,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    Markdown,
    Text,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum GroupBy {
    File,
}

// TODO: This is a test
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let stdin = io::stdin();
    if stdin.is_terminal() {
        return Ok(());
    }

    let Cli {
        output,
        group_by,
        detail,
    } = args;

    let mut todos = Vec::new();

    for path in stdin.lock().lines() {
        let path = path.expect("Error reading path");

        if let Ok(file) = std::fs::read_to_string(&path) {
            let mut new_items = todo_items_from_file(&file, &path, &detail)?;
            todos.append(&mut new_items)
        } else {
            continue;
        }
    }

    let mut group_spacer = "";
    if let Some(GroupBy::File) = group_by {
        for (path, todos) in &todos
            .into_iter()
            .chunk_by(|todo| todo.path.to_string_lossy().to_string())
        {
            println!("{}{}", group_spacer, path);
            let todos = todos.collect_vec();
            print_todos(&todos, &output).unwrap();

            group_spacer = "\n";
        }
    } else {
        print_todos(&todos, &output)?;
    }

    Ok(())
}

fn print_todos(todos: &[TodoItem], output: &OutputFormat) -> Result<()> {
    use OutputFormat::*;
    let printable = match output {
        Text => todos.iter().map(|t| t.to_string()).join("\n"),
        Markdown => todos.iter().map(|t| t.to_markdown()).join("\n"),
    };
    println!("{}", printable);
    Ok(())
}

fn todo_items_from_file(
    file: &str,
    path: &str,
    display_detail: &todo_item::DisplayDetail,
) -> Result<Vec<TodoItem>> {
    file.lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            TodoItem::maybe_from_line(line).map(|mut builder| {
                builder
                    .path(PathBuf::from(path))
                    .line_number(line_number)
                    .display_detail(display_detail.clone())
                    .build()
                    .context("Failed to build todoitem")
            })
        })
        .collect::<Result<Vec<_>>>()
}
