/// Function: add_five
///
/// # Arguments (num: u32)
///
/// # Returns u32
/// #Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
///
/**
* This is a multiline comment
*
*/

pub fn add_five(num: u32) -> u32 {
    // the -> u32 is the return type so we will return the inside of the function
    num + 5
}

#[cfg(test)] // this symbolizes that the following code is only for testing
mod test {
    // this is a test module
    use super::*; // we are importing everything from the parent module

    #[test] // this is a test function
    fn adds_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);
        println!("x and y are from test: {} {}", x, y);
        assert_eq!(y, 105); // this is the assertion to check if the function is working correctly
    }
}
