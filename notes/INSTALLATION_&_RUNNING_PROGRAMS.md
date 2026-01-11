# Installation

https://doc.rust-lang.org/book/ch01-01-installation.html

The installation will include the following:

* `rustup`: The Rust toolchain manager (i.e. a command line tool for managing Rust versions and associated tools).
* `rustc`: The Rust compiler.
* `cargo`: The Rust build system and package manager.

You can check the installed versions with the following commands:

```
rustup --version
rustc --version
cargo --version
```

## Updating and Uninstalling

https://doc.rust-lang.org/book/ch01-01-installation.html#updating-and-uninstalling

View `rustup` commands:

```
rustup
```

You can keep your Rust installation up to date with the latest version by running:

```
rustup update
```

There are new stable versions of Rust published every 6 weeks so this is a good habit.

---

# Scaffold, Build, Run Rust Programs

You can scaffold a new project with Cargo. Run the following commands from your project's root directory.

## Scaffold a Rust project

```
cargo new <project_name>
```

## Install dependencies

Install the latest version:

```
cargo add uuid
```

Install the latest version along with specific features:

```
cargo add uuid --features=fast-rng, v7
```

## Compile (or build) a program

These commands assume that you are using Cargo (recommended).

### Compile

Navigate to the project root directory (i.e. the directory that contains the `Cargo.toml` file) and run the following command:

```
// Compile the source code in debug mode
cargo build

// Run the compiled code
./target/debug/<your_project_name>
```

The `cargo build` command creates a debug version, which is an unoptimized version that compiles faster.

`cargo build` will look for a `main.rs` file inside the `src` directory and compile it. The compilation step will create a `target` directory with a bunch more files and directories inside. By default, the executable file will have the same name as the project root directory and it will be located inside the `target/debug` directory.

```
// Compile the source code in release mode
cargo build --release

// Run the compiled code
./target/release/<your_project_name>
```

The `--release` flag creates an optimized, production-ready binary version that takes longer to compile.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable, which is faster for development.

### Run an entire program

Navigate to the project root directory (i.e. the directory that contains the `Cargo.toml` file) and run the following command:

```
cargo run
```

`cargo run` will compile your program and run it. This command will find the executable file and run it, which is the same as running it manually by running `./target/debug/<executable_file>` in your terminal.

NOTE: You do _not_ have to run `cargo build` before running `cargo run`


### Run an individual file

Code files need to be inside the `src/bin` directory or the available binary targets need to be listed in the `Cargo.toml` file under the `[bin]` section.

Then you can navigate to the directory where your code file is located and run the code. For example, if your code file is named `a1.rs`, then you would run this:

```
cargo run --bin a1
```

NOTE: The `--bin` flag stands for binary.

*Source: https://code.visualstudio.com/docs/languages/rust#_hello-world*


### How to compile with rustc instead of Cargo

```
// Compile the source code
rustc main.rs

// Run the compiled code
./main
```
