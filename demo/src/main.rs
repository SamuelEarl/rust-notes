fn main() {
    let mut name: String = String::from("John"); // It is necessary to use the `mut` keyword in this case.
    greet(&mut name); // Call with `&mut String`. Ownership was not tranferred, so ownership does not need to be returned. You can still use `name` in this function.
    println!("{name}");
}

fn greet(name: &mut String) { // The `name` parameter is a mutable reference to the String.
    name.push_str(", how are you?"); // Since the parameter is a mutable String reference you can mutate the String reference and you don't have to return it.
}
