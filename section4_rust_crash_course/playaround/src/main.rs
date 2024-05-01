const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to the course: {}", OUR_COURSE);

    // Stack
    let x: i32;
    x = 10;
    println!("The value of x is: {}", x);

    let y: i32 = 4;
    println!("The value of y is: {}", y);

    // For loop
    for i in 0..=y { // 0..=y is a range from 0 to y inclusive
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }
    // Mutating a variable
    let mut z: i32 = 5;
    println!("The value of z was {}", z);
    z = 6;
    println!("The value of z is now {}", z);

    let freezing_temp: f64 = -2.4;
    println!("The freezing temperature is: {}", freezing_temp);

    let is_zero_remainder = 10 % 4 != 0;
    println!("Is there a zero remainder? {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is: {}", my_char);

    let emoji_char: char = '👻'; // can't have spaces otherwise it will be a string
    // you also cant have double quotes around it
    println!("emoji_char is: {}", emoji_char);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is: {:?}", my_floats); // {:?} is how will print an array

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is: {:?}", my_floats_new);

}