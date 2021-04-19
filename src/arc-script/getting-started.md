# Getting Started

This section explains how to create an Arc-Script project.

## Setup

Arc-Script is meant to be used alongside Rust, therefore the Rust compiler and Cargo package-manager must be installed on your computer, see this [tutorial](https://doc.rust-lang.org/cargo/getting-started/installation.html), or just run:

```sh
# Install cargo
curl https://sh.rustup.rs -sSf | sh
```

Then, download the arc-script template project:

```sh
git clone https://github.com/cda-group/arc-template
```

## Project Layout

The layout of the project is as follows:

```text
$ ls --recursive

arc-template/  # Project directory
  Cargo.toml   # Config file
  build.rs     # Build file
  src/         # Source directory
    main.rs    # Main file of Rust
    main.arc   # Main file of Arc-Script
  target/      # Build artefacts
```

## Arc-Script

Arc-Script code is placed in files with the `.arc` extension. Our Arc-Script contains a basic `Identity` task and a `test` function for using it. This file will be automatically compiled into Rust source that is placed in the `target/` directory. The artefacts can then be included from regular Rust files inside a Rust crate. For info about the Arc-Script language, please refer to the language reference.

```text
$ cat arc-template/src/main.arc

fun pipe(stream: ~i32 by unit) -> ~i32 by unit {
    stream |> Identity()
}

task Identity() ~i32 by unit -> ~i32 by unit {
    on event => emit event
}
```

## Rust

Our Rust code sets up an Arcon pipeline, which contains the Arc-Script pipeline, and passes it a vector of integers as input. The resulting stream is displayed in the console. Note that this code will in the future be abstracted away when building applications with Arc-Script.

```rust
$ cat arc-template/src/main.rs

use arc_script::arcorn::operators::*;
use arcon::prelude::{ArconTime, Pipeline};

mod script {
    // Include and encapsulate the Arc-Script inside a Rust module
    arc_script::include!("src/main.rs");
}

fn main() {
    // Setup an arcon pipeline
    let pipeline = Pipeline::default();

    // Create a data stream source
    let data = vec![1, 2, 3];

    let stream = pipeline
        .collection(data, |conf| {
            conf.set_arcon_time(ArconTime::Process);
        })
        .convert();

    // Connect the data stream with the arc-script
    let stream = script::pipe(stream);

    // Connect the data stream with a sink
    let mut pipeline = stream.to_console().build();

    // Execute the pipeline
    pipeline.start();
    pipeline.await_termination();
}
```

Any code written inside Rust can also be included into the Arc-Script. This will be covered in the language reference.

## Project Dependencies

The required project dependencies are as follows:

```toml
$ cat arc-template/Cargo.toml

[package]
name = "arc-script-template"
version = "0.0.0"
edition = "2018"

# Dependencies for running arc-scripts:
[dependencies]
arc-script = { version = "=0.0.0", git = "https://github.com/cda-group/arc" }
arcon      = { git = "https://github.com/segeljakt/arcon" }
prost      = { version = "0.7.0" }

# Dependencies for building arc-scripts:
[build-dependencies]
arc-script-build = { version = "=0.0.0", git = "https://github.com/cda-group/arc" }
```

## Build File

A build-file is also provided to re-compile the Arc-Script source when it changes.

```rust
$ cat arc-template/build.rs

use arc_script_build::Builder;

fn main() {
    // This pre-builds any file in the crate whose filename is `main.arc`.
    Builder::default().build();
}
```
