use std::{cell::OnceCell, fmt::Display, path::PathBuf, sync::OnceLock};

use derive_builder::Builder;
use regex::Regex;

#[derive(Builder, Debug)]
pub struct TodoItem {
    #[builder(setter(into))]
    pub path: PathBuf,
    pub task: String,
    pub line_number: usize,
    #[builder(setter(into))]
    pub display_detail: DisplayDetail,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DisplayDetail {
    FileAndLineNumber,
    JustTask,
}

impl TodoItem {
    pub fn maybe_from_line(line: &str) -> Option<TodoItemBuilder> {
        extract_task(line).map(|task| TodoItemBuilder::default().task(task).to_owned())
    }

    pub fn to_markdown(&self) -> String {
        format!("- [ ] {}", self)
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.display_detail {
            DisplayDetail::FileAndLineNumber => {
                write!(
                    f,
                    "{}:{}: {}",
                    self.path.display(),
                    self.line_number,
                    self.task
                )
            }
            DisplayDetail::JustTask => {
                write!(f, "{}", self.task)
            }
        }
    }
}

fn extract_task(line: &str) -> Option<String> {
    if line.is_empty() || !line.contains("TODO") {
        return None;
    }

    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| {
        regex::Regex::new(
            r"(?x)
            (?: //.*? 
              | /\*.*?\*/ 
              | \#.*? 
              | --.*? 
              | <!--.*?--> 
            )
            .*?
            \bTODO\b[:]?[ \t]*
            (.*)",
        )
        .expect("Failed to build regex")
    });

    regex
        .captures(line)
        .and_then(|c| c.get(1))
        .map(|m| strip_trailing_comment_symbols(m.as_str()).trim())
        .filter(|task| !task.is_empty())
        .map(String::from)
}

fn strip_trailing_comment_symbols(todo_text: &str) -> &str {
    todo_text.trim_end_matches(|c: char| !c.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_task() {
        [
            "// TODO: This is a test",
            "/// TODO: This is a test",
            "# TODO: This is a test",
            "Some code # TODO: This is a test",
            "-- TODO: This is a test",
            "-- TODO: This is a test  \"",
            "<!-- TODO: This is a test -->",
        ]
        .iter()
        .for_each(|example| {
            assert_eq!(
                extract_task(example),
                Some("This is a test".to_string()),
                "Failed on: {}",
                example
            )
        })
    }

    #[test]
    fn test_display_todo() {
        let todo = TodoItemBuilder::default()
            .path(PathBuf::from("some/path"))
            .task("Do something".to_string())
            .line_number(42)
            .display_detail(DisplayDetail::FileAndLineNumber)
            .build()
            .unwrap();

        assert_eq!(todo.to_string(), "some/path:42: Do something");
        assert_eq!(todo.to_markdown(), "- [ ] some/path:42: Do something");

        let todo = TodoItemBuilder::default()
            .path(PathBuf::from("some/path"))
            .task("Do something".to_string())
            .line_number(42)
            .display_detail(DisplayDetail::JustTask)
            .build()
            .unwrap();

        assert_eq!(todo.to_string(), "Do something");
        assert_eq!(todo.to_markdown(), "- [ ] Do something");
    }
}
