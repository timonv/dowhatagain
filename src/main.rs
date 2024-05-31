use anyhow::{Context as _, Result};
use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    io::{self, BufRead, BufReader, IsTerminal},
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

    let input = BufReader::new(stdin);

    let Cli {
        output,
        group_by,
        detail,
    } = args;

    let todos: Vec<TodoItem> = input
        .lines()
        .par_bridge()
        .try_fold(Vec::new, |mut todos, path| {
            let path = path?.to_owned();

            // Ignore files that don't exist or are not files
            let result = std::fs::metadata(&path).and_then(|m| {
                if m.is_file() {
                    Ok(m)
                } else {
                    Err(std::io::Error::new(io::ErrorKind::Other, "Not a file"))
                }
            });
            if result.is_err() {
                return Ok(todos);
            }

            let file = std::fs::read_to_string(&path)?;
            todo_items_from_file(&file, &path, &detail).map(|mut new_items| {
                todos.append(&mut new_items);
                todos
            })
        })
        .try_reduce(Vec::new, |mut acc, todos| {
            acc.extend(todos);
            Ok(acc)
        })?;

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
