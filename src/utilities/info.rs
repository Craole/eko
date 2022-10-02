#![allow(dead_code)]

/// Check the type of a variable
pub fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
