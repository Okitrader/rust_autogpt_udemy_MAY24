/// Adds left and right
///
/// # Arguments
///
/// * `left` - The left side of the equation
/// * `right` - The right side of the equation
/// * 'usize' - is the type of the arguments that returns the function
/// # Example
/// ```
/// # use my_library::add;// Assuming the crates's name is my_library
/// let l: usize = 20;
/// let r: usize = 5;
/// assert_eq!(add(l, r), 25);
/// ```
pub fn add(left: usize, right: usize) -> usize {
//    // the -> u32 is the return type so we will return the inside of the function
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
