# Timestamp

3:34:02 => Exercise - Impl

# Rust Notes

Many of these notes are taken from this video: [Rust 101 Crash Course: Learn Rust (6 HOURS!) + 19 Practice Exercises | Zero To Mastery](https://www.youtube.com/watch?v=lzKeecy4OmQ)

## Scaffold, Build, Run Rust Programs

Run the following commands from your project's root directory.

### Scaffold a Rust project

```
cargo new <project_name>
```

### Compile (or build) a program

```
cargo build
```

### Run an entire program

```
cargo run
```

`cargo run` will look for a `main.rs` file and run that code. (Does the `main.rs` file have to be inside a `src` directory?)

NOTE: You do _not_ have to run `cargo build` before running `cargo run`


### Run an individual file

Code files need to be inside the `src/bin` directory or the available binary targets need to be listed in the `Cargo.toml` file under the `[bin]` section.

Then you can navigate to the directory where your code file is located and run the code. For example, if your code file is named `a1.rs`, then you would run this:

```
cargo run --bin a1
```

NOTE: The `--bin` flag stands for binary.

*Source: https://code.visualstudio.com/docs/languages/rust#_hello-world*


## Update Rust

You can keep your Rust installation up to date with the latest version by running:

```
rustup update
```

There are new stable versions of Rust published every 6 weeks so this is a good habit.

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

# Structs vs Tuples

* Structs are like objects in JavaScript and Dictionaries in Python.
* It is recommended to use structs when you are working with a data structure that has more than 2 or 3 fields because structs have named values, which can help you keep your code organized.

# Expressions

* Rust is an expression-based language, which means that you can set a variable to equal the result of a condition evaluation (e.g. with an `if else` or `match` statement). 
* This is similar to JavaScript ternary expressions. 
* See the `expressions.rs` file for an example.

# Implementations (impl)

* `impl` blocks are similar to methods in class-based programming languages.
