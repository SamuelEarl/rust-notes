// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(a: i32, b: i32) -> i32 {
  a + b
}

fn print_answer(answer: i32) {
  println!("{answer:?}");
}

fn main() {
  let answer = sum(5, 3);
  let nums = sum(12, 4);
  print_answer(answer);
}
