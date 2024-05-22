# Todors

Simple tool for filtering todos from a piped in list of files and outputting to various formats.

TODO comments in code have a tendency to be just left there yet they also have their value when building complicated things.

![image](https://github.com/timonv/todors/assets/49373/226d6307-4dc4-46f1-9806-e3741f728996)


## Installation

Manual:

- Clone the repository
- Install the rust toolchain
- Run `cargo install --path .`

Via cargo:
`cargo install todors`

## Example usage

Get all TODOs in a repository:
`fd . | todors`

Output:

```
// File: main.rs
TODO: Do thing
// File: other.rs
TODO: Other thing
```

Get all TODOs for the current changeset and output to markdown:
`git diff master --name-only | todors --output markdown`

```markdown
- [ ] file.rs:3 Do thing
- [ ] other_file.rs:4 Do other thing
```

Great for PR checks, commit hooks and custom workflows.
