mod my_funcs; // we are importing the module
mod other_funcs; // we are importing the module

use crate::my_funcs::add_five; // we are importing the function from the module
                               // use crate::my_funcs::subtract_ten; // we are importing the function from the module
                               // or we can use the following: use crate::my_funcs::*; // we are importing all the functions from the module
                               // or we can use the following: use crate::my_funcs::{add_five, subtract_ten}; // we are importing the functions from the module
use crate::other_funcs::minus_funcs::subtract_ten; // we are importing the function from the module

// Everything is defaulted to immutable
fn main() {
    let mut x: u32 = 50; // mutable variable
    println!("The value of x is: {}", x); // the semicolon is here because we are not returning

    let y: u32 = add_five(x); // we are passing x to the function
    println!("The value of y is: {}", y);

    let z: u32 = subtract_ten(x); // we are passing x to the function
    println!("The value of z is: {}", z);

    x = 70;
    println!("The value of x is: {}", x);
}
