use std::ops::{Add, Sub, Mul};
pub fn num_sum<T: Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Copy>(a: T, b: T) -> T {
    return (a+b)*(a-b);
}

