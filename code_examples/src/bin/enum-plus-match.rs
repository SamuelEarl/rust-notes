// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}

fn main() {
    which_way(Direction::Up);
    which_way(Direction::Down);
    which_way(Direction::Left);
    which_way(Direction::Right);
}
