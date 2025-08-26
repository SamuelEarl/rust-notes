# Rust Notes

These notes are taken from [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book.

---

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

## Compile (or build) a program

These commands assume that you are using Cargo (recommended).

### Compile

```
cargo build
```

`cargo build` will look for a `main.rs` file inside the `src` directory and compile it. The compilation step will create a `target` directory with a bunch more files and directories inside. By default, the executable file will have the same name as the project root directory and it will be located inside the `target/debug` directory.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable, which is faster for development.

### Run an entire program

```
cargo run
```

`cargo run` will build and run a project. This command will find the executable file and run it, which is the same as running it manually by running `./target/debug/<executable_file>` in your terminal.

NOTE: You do _not_ have to run `cargo build` before running `cargo run`


### Run an individual file

Code files need to be inside the `src/bin` directory or the available binary targets need to be listed in the `Cargo.toml` file under the `[bin]` section.

Then you can navigate to the directory where your code file is located and run the code. For example, if your code file is named `a1.rs`, then you would run this:

```
cargo run --bin a1
```

NOTE: The `--bin` flag stands for binary.

*Source: https://code.visualstudio.com/docs/languages/rust#_hello-world*


### Compiling with rustc instead of Cargo

```
// Compile the source code
rustc main.rs

// Run the compiled code
./main
```

---

# String Interpolation

As of Rust 1.58, you can take advantage of [captured identifiers in format strings](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0/#captured-identifiers-in-format-strings).

# Macros

An exclamation point is used to call/invoke a macro (e.g. `println!("How are you?)`).

## Debugging

The `:?` placeholder is used for debugging by printing the debug representation of a value.

```rs
fn main() {
    let sum = add(1, 3);
    println!("The sum is {sum:?}");
}
```

# else if vs match

* Prefer `match` over `else if` when working with a single variable.
* `match` considers all possibilities and creates more robust code  (i.e. there is greater confidence that your code is error free).
* Use an underscore as a match "catch all" (similar to `else` clauses).
* When `match` is used with enums, your programs can be more robust.

# Structs & Implementations (impl)

* Structs are sort of like class variables in class-based programming languages and `impl` blocks are similar to methods in class-based programming languages.
* You can also think of structs as sort of like objects in JavaScript and Dictionaries in Python.
* It is recommended to use structs when you are working with a data structure that has more than 2 or 3 fields because structs have named values, which can help you keep your code organized.

# Expressions

* Rust is an expression-based language, which means that you can set a variable to equal the result of a condition evaluation (e.g. with an `if else` or `match` statement). 
* This is similar to JavaScript ternary expressions. 
* See the `expressions.rs` file for an example.
