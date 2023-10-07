# Silly workspace

## Setup

The project uses Python 3 and Rust, so both need to be installed. You can use the Python binary in the provided `python_path`:

```toml
python_path = "/opt/homebrew/bin/python3"
```

## Tasks

The project defines the following tasks in the Maidfile:

### Clean

The `clean` task removes the `project/bin` directory and recreates it.

To run this task, use the command `maid clean`.

### Build

The `build` task first executes the `clean` task, then builds the Rust
library in release mode and moves the output to the `project/bin` directory.

To run this task, use the command `maid build`.

### Dev

The `dev` task builds the project and runs the Python script `project/main.py`.

To run this task, use the command `maid dev`.

### Run

The `run` task directly executes the Python script `project/main.py`.

To run this task, use the command `maid run`.

### Unrelated

Check out [maid](https://github.com/exact-labs/maid), the task runner used here.
