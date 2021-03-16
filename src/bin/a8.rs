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
    Grapefruit,
    Orange,
    Lemon,
    Lime
}

struct Drink {
    flavor: Flavor,
    volume: f64
}

fn drink_info(drink: Drink) {
    match drink.flavor {
        Flavor::Grapefruit => println!("flavor: grapefruit"),
        Flavor::Orange => println!("flavor: orange"),
        Flavor::Lemon => println!("flavor: lemon"),
        Flavor::Lime => println!("flavor: lime"),
    }

    println!("oz: {:?}", drink.volume);
}

fn main() {
    let lemon = Drink {
        flavor: Flavor::Lemon,
        volume: 12.0
    };

    let orange = Drink {
        flavor: Flavor::Orange,
        volume: 20.0
    };

    drink_info(lemon);
    drink_info(orange);
}
