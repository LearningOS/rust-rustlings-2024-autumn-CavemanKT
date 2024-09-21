// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.
use std::any::type_name;

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40];// TODO: declare your vector here with the macro for vectors
    println!("a: {:?}", a);
    println!("a: {:?}", type_name_of(&a));
    println!("v: {:?}", v);
    println!("v: {:?}", type_name_of(&v));
    (a, v)
}

fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
