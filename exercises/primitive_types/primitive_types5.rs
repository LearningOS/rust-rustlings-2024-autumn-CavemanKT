// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.
use std::any::type_name;

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let /* your pattern here */(name, age) = cat;
    println!("type_name_of(&cat) = {}", type_name_of(&cat));
    println!("{} is {} years old.", name, age);
}

fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}