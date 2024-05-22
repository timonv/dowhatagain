use std::{
    fmt::Display,
    io::{self, BufRead},
    path::PathBuf,
};

use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct TodoItem {
    #[builder(setter(into))]
    path: PathBuf,
    task: String,
    line_number: usize,
}

impl TodoItem {
    pub fn maybe_from_line(line: &str) -> Option<TodoItemBuilder> {
        extract_task(line).map(|task| TodoItemBuilder::default().task(task).to_owned())
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "- [ ] {}:{} {}",
            self.path.display(),
            self.line_number,
            self.task,
        )
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.path.display(),
            self.line_number,
            self.task
        )
    }
}

fn extract_task(line: &str) -> Option<String> {
    let regex = regex::Regex::new(
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
    .unwrap();

    regex
        .captures(line)
        .and_then(|c| c.get(1))
        .map(|m| strip_trailing_comment_symbols(m.as_str()))
        .map(|task| task.trim().to_string())
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
}
