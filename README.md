# Todors

Simple tool for filtering todos from a piped in list of files and outputting to various formats.

TODO comments in code have a tendency to be just left there yet they also have their value when building complicated things.

## Installation

tbd

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
`git diff master --name-only | todo`

```
**file: main.rs**
- [ ] Do thing

**file: other.rs**
- [ ] Do other thing
```

Great for PR checks, commit hooks and custom workflows.
