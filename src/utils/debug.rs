use std::fmt::Debug;

pub fn debug<T: Debug>(value: T) {
    println!("{:?}", value);
}
