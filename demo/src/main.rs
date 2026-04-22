fn main() {
    let mut name: String = String::from("John"); // It is necessary to use the `mut` keyword in this case.
    let ref1 = &mut name;
    let ref2 = &name;
    ref1.push_str(" hello");
    println!("{ref2}");
}
