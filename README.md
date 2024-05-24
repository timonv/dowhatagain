# Dowhatagain

Simple tool for filtering todos from a piped in list of files and outputting to various formats.

TODO comments in code have a tendency to be just left there yet they also have their value when building complicated things.

![image](https://github.com/timonv/todors/assets/49373/226d6307-4dc4-46f1-9806-e3741f728996)

## Installation

Manual:

- Clone the repository
- Install the rust toolchain
- Run `cargo install --path .`

Via cargo:
`cargo install dowhatagain`

## Features

- Output in Markdown and plain text
- Group by filename
- Simple display or detailed with filename and line number
- Way faster than needed using buffered parallel processing

## Example usage

Get all TODOs in a repository:

```
$ fd . | dowhatagain

file1.rs:3: Do thing
file2.rs:4: Other thing
```

Get all TODOs for the current changeset and output to markdown:
`git diff master --name-only | dowhatagain --output markdown --group-by file --detail just-task`

```markdown
file.rs

- [ ] Do thing
- [ ] Do other thing

other_file.rs

- [ ] Do thing
- [ ] Do other thing
```

Great for PR checks, commit hooks and custom workflows.
