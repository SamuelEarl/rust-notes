// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Lemon,
    Lime,
    Raspberry,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Lemon => println!("Flavor: {0}, Ounces: {1}", "lemon", drink.fluid_oz),
        Flavor::Lime => println!("Flavor: {0}, Ounces: {1}", "lime", drink.fluid_oz),
        Flavor::Raspberry => println!("Flavor: {0}, Ounces: {1}", "raspberry", drink.fluid_oz),
    }
}

fn main() {
    let drink1 = Drink {
        flavor: Flavor::Lemon,
        fluid_oz: 12.0,
    };
    print_drink(drink1);
}
