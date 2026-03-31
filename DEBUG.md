Many of the complex data types (e.g. arrays, tuples, ranges) do NOT implement the Display trait, but they do implement the Debug trait. That means that you will not be able to inspect the value of a complex data type using the Display trait (e.g. `println!("{person}");`), but you will be able to inspect it using the Debug trait (e.g. `println!("{person:?}");).

The following format specifiers allow you to use the Debug trait when inspecting code: 

* `{:?}` (The debug format spacifier)
* `{:#?}` (The debug format spacifier with pretty printing)

```rust
fn main() {
    let employee = ("Molly", 32, "Marketing");

    // Each format specifier option has two options for string interpolation:

    println!("{:?}", employee);
    println!("{employee:?}");

    println!("{:#?}", employee);
    println!("{employee:#?}");
}
```

You can also use the `dbg!()` macro to get specific line numbers and more detailed output.
