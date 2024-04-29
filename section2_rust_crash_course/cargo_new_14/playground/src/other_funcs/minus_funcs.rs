// src/other_funcs/minus_funcs.rs
pub fn subtract_ten(num: u32) -> u32 {
    num - 10
}

#[cfg(test)] // this symbolizes that the following code is only for testing
mod test {
    // this is a test module
    use super::*; // we are importing everything from the parent module

    #[test] // this is a test function
    fn subtracts_ten_tests() {
        let x: u32 = 100;
        let y: u32 = subtract_ten(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 99); // this is the assertion to check if the function is working correctly
    }
}
