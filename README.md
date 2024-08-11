# TODO List Application

A command-line application for managing your tasks efficiently.

## Features

- Task components: title, description, creation date, category, completion status
- Add, edit, and view tasks
- View a list of current tasks with the ability to filter by a given predicate

## Installation

Ensure you have Rust installed on your system. Then, clone this repository and build the project:

```bash
git clone repo
cd to repo folder
cargo build --release
cd target/release
```

The executable will be available in `target/release/todolist`.

## Usage

### Add a new task

```bash
./todolist add <name> <description> <date> <category>
```

Example:
```bash
./todolist add "Buy groceries" "Get milk, eggs, and bread" "15-08-2023 18:00" "Shopping"
```

### Mark a task as done

```bash
./todolist done <name>
```

### Update a task

```bash
./todolist update <name>
```

After this command, you'll be prompted to enter new values for each field interactively.

### Delete a task

```bash
./todolist delete <name>
```

### Select tasks

To view all tasks:
```bash
./todolist select \*
```

To filter tasks based on a predicate:
```bash
./todolist select "* where <predicate>"
```

Example:
```bash
./todolist select "* where date < 2023-12-31 00:00 and category=work and status=on and description like project"
```

#### Predicate syntax:

- Available comparison operators: `<`, `<=`, `=`, `>=`, `>`
- Conditions can be combined using `and`
- Use `like` keyword for substring matching in text fields

## Development

This project is written in Rust. To contribute or modify the code, make sure you have Rust installed on your system.

To run tests:
```bash
cargo test
```

To build the project:
```bash
cargo build
```

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
