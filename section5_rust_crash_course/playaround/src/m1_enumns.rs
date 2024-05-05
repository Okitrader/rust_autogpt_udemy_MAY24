// from the m1_enumns.rs file
// Enums are types which have a few definite values
#[derive(Debug)] // This is so we can inspect the state of the enum
enum CarColor {
    Red,
    Green,
    Blue,
    Silver,
}

fn create_car_color_blue() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    my_car_color
}

#[derive(Debug)]
enum GivenResult<T, E> {
    // I will tell T or E and what type - this is a generic type enum
    Ok(T),
    Err(E),
}

#[derive(Debug)]
enum GivenOption<T> {
    // T is generic so we can use this for many use cases in other parts of the code
    None,
    Some(T),
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under five".to_string())
    }
}

// Built in Result Enum
fn check_under_five_built_in(num_check: u8) -> Result<u8, String>{ // Result is a built in enum
    if num_check < 5 {
        Ok(num_check) // no need to use Result::Ok because it is built in
    } else {
        Err("Not under five".to_string()) //
    }
}

// Option Enum
fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0; // Changed 10 to 10.0
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

// Built in Option Enum
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0; // Changed 10 to 10.0
    if remainder != 0.0 {
        Some(remainder) // no need to use Option::Some because it is built in
    } else {
        None // no need to use Option::None because it is built in
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_color: CarColor = create_car_color_blue();
        dbg!(car_color); // we can do this because we have #[derive(Debug)] on the enum above

        let under_five_result: GivenResult<u8, String> = check_under_five(18);
        dbg!(under_five_result);

        let under_five_result_built_in: Result<u8, String> = check_under_five_built_in(20);
        dbg!(under_five_result_built_in);

        let remainder: GivenOption<f32> = remainder_zero(10.0); // Changed 12 to 12.0
        dbg!(remainder);

        let remainder_built_in: Option<f32> = remainder_zero_built_in(1185.0); // Changed 12 to 12.0
        dbg!(remainder_built_in);
    }
}