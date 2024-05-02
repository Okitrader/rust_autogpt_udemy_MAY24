const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
//     println!("Welcome to the course: {}", OUR_COURSE);
//
//     // Stack
//     let x: i32;
//     x = 10;
//     println!("The value of x is: {}", x);
//
//     let y: i32 = 4;
//     println!("The value of y is: {}", y);
//
//     // For loop
//     for i in 0..=y { // 0..=y is a range from 0 to y inclusive
//         if i != 4 {
//             print!("{}, ", i);
//         } else {
//             println!("{}", i);
//         }
//     }
//     // Mutating a variable
//     let mut z: i32 = 5;
//     println!("The value of z was {}", z);
//     z = 6;
//     println!("The value of z is now {}", z);
//
//     let freezing_temp: f64 = -2.4;
//     println!("The freezing temperature is: {}", freezing_temp);
//
//     let is_zero_remainder = 10 % 4 != 0;
//     println!("Is there a zero remainder? {}", is_zero_remainder);
//
//     let my_char: char = 'z';
//     println!("my_char is: {}", my_char);
//
//     let emoji_char: char = '👻'; // can't have spaces otherwise it will be a string
//     // you also cant have double quotes around it
//     println!("emoji_char is: {}", emoji_char);
//
//     let my_floats: [f32; 10] = [0.0; 10];
//     println!("my_floats is: {:?}", my_floats); // {:?} is how will print an array
//
//     let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
//     println!("my_floats_new is: {:?}", my_floats_new);

    let name: &str = "Ajax";
    println!("My name is: {}", name);

    let dynamic_name: String = String::from("Ajax Okitrader");
    // String::from is a function that creates a new string
    println!("dynamic_name is: {:?}", dynamic_name); // {:?} is how will print a string
    println!("my dynamic_name stored in memory is: {:p}", &dynamic_name);
    // the {:p} is a pointer to the memory location of the variable

    let str_slice: &str = &dynamic_name[0..5];
    println!("str_slice is: {}", str_slice);

    let mut chars: Vec<char> = Vec::new(); // Vec is a growable array
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push( 'l');
    chars.push( 'o');
    chars.push( '!');
    println!("chars is: {:?}", chars);
    //debug print. it saves time from writing. also prints the location in the code
    dbg!(&chars); // this is a debug print that prints the line number

    let removed_char: char = chars.pop().unwrap();
    //.unwrap() is used to get the value out of the Option type need it because pop returns an Option
    println!("removed_char is: {:?}", removed_char);

    // iterating through a vector
    chars.iter().for_each(|c| println!("char is {}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o', '!'];
    dbg!(&chars_again); // prints error if println! is used instead of dbg!

    let collected: String = chars_again.iter().collect(); // collecets and stores in variable
    dbg!(&collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            println!(", world");
        }
    }

}

