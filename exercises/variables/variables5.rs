// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

use std::any::type_name;

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", type_name_of(&number));
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", type_name_of(&number));
}

// Helper function to get the type of a variable
fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}