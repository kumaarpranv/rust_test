use crate::generics::num_sum;

#[test]
fn sample_test() {
    assert_eq!("hello", "hello");
}

#[test]
fn int32_test() {
    assert_eq!(num_sum(2,3), -5);
}
