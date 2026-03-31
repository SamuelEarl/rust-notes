# Strings

## Rust has two core string types and reference versions of those string types

### String Slice (or Borrowed String)

A `str` is a hardcoded, read-only piece of text that is encoded in the compiled executable (binary) file. An example of this would be creating paragraph text in a Dioxus app. 

A `&str` is a reference to a `str`. You typically won't be interacting directly with `str` types. You will usually interact with `&str` types, which are references to the text in the compiled executable (binary) file that has been loaded in memory (i.e. the binary file has been loaded in memory and `&str`s are references to something like `"John Doe"` in the binary file).

1. **Description:** This type is an immutable reference to a value that is stored in the binary executable file at compile time. This is because the value is already known at compile time. These strings are extremely fast because they are just pointers and a length.
2. **Example:** `let name: &str = "John";`
3. **Size:** Fixed size. You cannot change its length.
4. **Ownership:** Borrowed (reference).
5. **Coercion:** Cannot turn into `String` for free (i.e. cannot turn into `String` without taking up a more substantial amount of memory).

### Owned String

A `String` is a dynamic piece of text stored on the heap at runtime.

A `&String` (ref `String`) is a reference to a heap `String`.

1. **Description:** This is a heap-allocated, growable, and mutable string. It is essentially a `Vec<u8>`.
2. **Examples:**
    * Using the `String::from()` method (commonly used):
    ```rust
    let name: String = String::from("John");
    ```
    * Using the `String::new()` method:
    ```rust
    let mut name: String = String::new();
    name.push_str("John");
    ```
3. **Size:** Growable/Modifiable. You can push characters, truncate it, or concatenate it.
4. **Ownership:** The variable owns the data. When the variable goes out of scope, the memory is freed.
5. **Coercion:** Can easily turn into `&str`.

<br />

## When should you use String Slices vs Owned Strings?

Choosing between `&str` and `String` usually comes down to one question: **Who should "own" the memory?** In Rust, owning memory means you are responsible for cleaning it up when you're done. Borrowing a slice means you are just looking at memory someone else is currently managing.

### Use `&str` (Slice) when...

You only need to read the data or perform an operation that doesn't require keeping the data forever.

1. **Function Arguments:** This is the "Golden Rule" of Rust functions. Always prefer `&str` for input parameters. It allows the caller to pass in a `&str`, a `&String` (i.e. a reference to a `String`), or a literal.

```rust
fn main() {
    let name: &str = "John";
    greet(name); // Call with &str
    println!("NAME: {name}");
}

fn greet(name: &str) { // Flexible: accepts &str, &String or "literal"
    println!("Hello, {name}!");
}
```

```rust
fn main() {
    let name: String = String::from("John");
    greet(&name); // Call with &String

    println!("NAME: {name}");
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

```rust
fn main() {
    greet("John"); // Call with literal
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

2. **Hardcoded Constants:** You are using text literals defined in your code (which are implicitly `'static`).

3. **Sub-strings:** You want to look at a small part of a larger string without copying the whole thing into new memory.

```rust
let full_path = String::from("/user/home/file.txt");
let file_extension = &full_path[16..20]; // Points to ".txt" inside full_path
```

### Use `String` (Owned) when...

You need to **mutate**, **store**, or **move** the data across boundaries where the original source might disappear.

1. **Dynamic Creation:** You are building a string at runtime (e.g., `format!("Hello, {}", name)`).

2. **Struct Fields:** You are defining a data model or a Struct that needs to hold onto its own data.

```rust
struct User {
    username: String, // The struct "owns" this text
}
```

3. **Taking Ownership:** You are writing a function that _must_ have its own copy of the data because it plans to change it or send it to another thread.
```rust
fn main() {
    let name: String = String::from("John");
    greet(name);
}

fn greet(mut name: String) {
    name.push_str(" Doe");
    println!("Hello, {name}!");
}
```

```rust
fn main() {
    let name: String = String::from("John");
    greet(name.clone()); // This creates a copy of name.
    println!("NAME: {name}"); // name is still in scope and can be used here.
}

fn greet(mut name: String) {
    name.push_str(" Doe");
    println!("Hello, {name}!");
}
```

4. **User Input:** Receiving data from a form, a database, or an API.

<br />

### Summary Rule of Thumb

* If you are **borrowing** data in order to **read** it, then use `&str`.
    * To borrow (verb) a value in Rust means to create a reference (noun) to that value.
    * A reference is one type of a pointer. A reference is guaranteed to point to a location on the heap that has a valid value (i.e. it is a safe pointer).
    * You can use the dereference operator (*) to access the data at the memory address that the reference points to. However, if you are interpolating a `String` inside of another string (e.g. when you print a `String` inside of another string while debugging), then you can simply use the original reference, instead of the dereferenced value, because Rust implements the `Display` trait for references. For example:
    ```rust
    fn main() {
        let name: String = String::from("John");
        println!("NAME: {name}");
        let name_ref = &name;
        println!("NAME REF: {name_ref}");
        println!("NAME DEREF: {}", *name_ref);
    }
    // NAME: John
    // NAME REF: John
    // NAME DEREF: John
    ```
* If you are taking **ownership** of data (via a `move` or `clone` operation) in order to **mutate** it, then use `String`.
    * **`move`:** A `move` is the default operation that occurs when a `String` variable is reassign to a new variable. For example:
    ```rust
    let person1: String = String::from("John"); // When the value of person1 is reassigned to person2 (in the next line), then person1 goes out of scope and becomes invalid.

    let person2: String = person1; // There can only be one owner at a time. Ownership has been moved to person2, so person2 is responsible for deallocating (i.e. dropping) the string's heap memory when person2 goes out of scope.
    ```
    * **`copy`:** `String`s do not implement the `Copy` trait (only primitive data types do), but `String`s do implement the `Clone` trait. So if you want to copy a `String` to another variable (instead of moving ownership to the other variable), then you have to explicitly copy it with the `clone` method. For example:
    ```rust
    let person1: String = String::from("John");
    let person2: String = person1.clone(); // Now there are two separate locations on the heap that each contain the String value of "John" and each location has its own, separate variable that points to it.
    ```

<br />

### Usage Comparison for Dioxus Development

| **Scenario** | **Use `&str`** | **Use `String`** |
| ------------ | -------------- | ---------------- |
| **Component Props** | When the prop is a hardcoded literal (use `'static str`). | When the component needs to own the data (common in Dioxus 0.7). |
| **State (`Signal`)** | Rarely, unless it's a constant reference. | Almost always. Signals usually own their data. |
| **Event Handlers** | When comparing a value (e.g., `if input == "submit"`). | When processing form data or input. |

<br />

## How to Convert Between String Types

### From `&str` to `String`

```rust
let version1 = "hello".to_string();
let version2 = String::from("hello");
```

### From `String` to `&str`:

```rust
let owned: String = String::from("hello");
let slice: &String = &owned; // Using the borrow operator (&) "borrows" the value as a slice. Notice that the data type is a reference to a String, not the String itself.
```

<br />

## What is the difference between the `&str` and `&'static str` types?

The difference between `str` and `&'static str` comes down to lifetimes. In Rust, every reference has a lifetime that defines how long that reference is valid.

### `&'static str` (The "Forever" String)

The `'static` lifetime is a special reserved lifetime in Rust. It means the data pointed to by the reference will persist for the entire duration of the program.

* **Where it lives:** Usually embedded directly in the compiled binary's read-only data segment.

* **How it's created:** Any string literal you type in your code is automatically a `&'static str`.

* **Safety:** Since it never goes away, you can pass it anywhere, to any thread, or store it in a global variable without worrying about it becoming invalid.

```rust
let literal: &'static str = "I am stored in the binary";
```

### `str` (The "Temporary" String)

A `str` without the explicit `'static` marker is a generic string slice. Its lifetime is "elided" (hidden) or tied to a specific scope. It is a borrow of some other string data.

* **Where it lives:** It could point to a `String` on the heap, a buffer on the stack, or even a sub-section of a `'static` string.

* **How it's created:** Usually by borrowing an owned `String`.

* **Constraint:** It cannot outlive the data it points to. If the owner of the data is dropped, the `str` becomes illegal to use.

```rust
{
    let owned_string = String::from("I live on the heap");
    let str_slice: &str = &owned_string; // slice is valid here
} 
// owned_string is dropped here.
// str_slice would be a "dangling pointer" if Rust let you use it here, but Rust does not allow that.
```

### Key Differences at a Glance

| **Feature** | **`&'static str`** | **`&str`** |
| ----------- | ---------------- | -------- |
| **Duration** | Lasts until the program ends. | Lasts only as long as its owner. | 
| **Storage** | Read-only memory (Binary).| Anywhere (Heap, Stack, or Binary). |
| **Flexibility** | High (can be used anywhere). | Low (must stay within its "allowed" scope). |
| **Coercion** | Can be downgraded to a regular `&str`. | Cannot be upgraded to `'static` (unless leaked). |

<br />

## String Interpolation

As of Rust 1.58, you can take advantage of [captured identifiers in format strings](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0/#captured-identifiers-in-format-strings).
