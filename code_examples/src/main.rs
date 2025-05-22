fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let name = String::from("Sam");
    println!("Hello, {name}!");

    let sum = add(1, 3);
    println!("The sum is {sum}");
    // The :? placeholder is used for debugging by printing the debug representation of a value.
    println!("The sum is {sum:?}");
    println!("Hi there");
}


// fn main() {
//     let age = 15;
//     if age >= 21 {
//         println!("ok to purchase");
//     }
//     else {
//         println!("cannot purchase");
//     }
// }
