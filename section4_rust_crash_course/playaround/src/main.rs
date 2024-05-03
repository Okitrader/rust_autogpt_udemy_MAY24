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

    // let name: &str = "Ajax";
    // println!("My name is: {}", name);
    //
    // let dynamic_name: String = String::from("Ajax Okitrader");
    // // String::from is a function that creates a new string
    // println!("dynamic_name is: {:?}", dynamic_name); // {:?} is how will print a string
    // println!("my dynamic_name stored in memory is: {:p}", &dynamic_name);
    // // the {:p} is a pointer to the memory location of the variable
    //
    // let str_slice: &str = &dynamic_name[0..5];
    // println!("str_slice is: {}", str_slice);
    //
    // let mut chars: Vec<char> = Vec::new(); // Vec is a growable array
    // chars.insert(0, 'h');
    // chars.insert(1, 'e');
    // chars.insert(2, 'l');
    // chars.push( 'l');
    // chars.push( 'o');
    // chars.push( '!');
    // println!("chars is: {:?}", chars);
    // //debug print. it saves time from writing. also prints the location in the code
    // dbg!(&chars); // this is a debug print that prints the line number
    //
    // let removed_char: char = chars.pop().unwrap();
    // //.unwrap() is used to get the value out of the Option type need it because pop returns an Option
    // println!("removed_char is: {:?}", removed_char);
    //
    // // iterating through a vector
    // chars.iter().for_each(|c| println!("char is {}", c));
    //
    // let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o', '!'];
    // dbg!(&chars_again); // prints error if println! is used instead of dbg!
    //
    // let collected: String = chars_again.iter().collect(); // collecets and stores in variable
    // dbg!(&collected);
    //
    // for c in chars_again {
    //     print!("{}", c);
    //     if c == 'o' {
    //         println!(", world");
    //     }
    // }
    //
    // //Closures
    // let num: i32 = 5;
    // let add_num = |n: i32| n + num; //removed from scope when the function is done
    // let new_num: i32 = add_num(7);
    // dbg!(new_num);

    // Number Literals (from the Rust book)
    // println!("Big number is: {}", 98_222_000);
    // println!("Hex is: {}", 0xff); // Hex
    // println!("Octal is: {}", 0o77); // Octal
    // println!("Binary is: {}", 0b1111_0000); // Binary
    // println!("Byte (u8) is: {}", b'A'); // Byte (u8)
    //
    // // Raw - String Literal
    // let text: &str = r#"{Message : "Rust is Awesome}"#; // useful if we have a lot of quotes - raw string
    // dbg!(text);

    // Binary
    let a: u8 = 0b1010_1010;
    let b: u8 = 0b0101_0101;
    println!("A's value is {}", a);
    // println!("B's value is {}", b);
    //
    //
    println!("A's binary: {:08b}", a);
    // println!("B's binary: {:08b}", b);
    // // Logic Gates
    // println!("AND: {:08b}", a & b); //prints a and b in binary. both to be 1 to be 1 which is 0b0000_0000
    // println!("OR: {:08b}", a | b); //prints a or b in binary. either to be 1 to be 1 which is 0b1111_1111
    // println!("XOR: {:08b}", a ^ b); //prints a xor b in binary. either to be 1 to be 1 which is 0b1111_1111
    // println!("NOT: {:08b}", !a); //prints not a in binary. which is 0b0101_0101
    // println!("Shift Left: {:08b}", a << 1); //prints a shifted left by 1 in binary. which is 0b0101_0100
    // println!("Shift Right: {:08b}", a >> 1); //prints a shifted right by 1 in binary. which is 0b1010_101

    //Bitwise Operations
    println!("a << 1: {:08b}", a << 1); //shifts a to the left by 1
    println!("a << 1 {}", a << 1);
    println!("a >> 1: {:08b}", a >> 1); //shifts a to the right by 1
    println!("a >> 1 {}", a >> 1);

    // Little Endian and Big Endian
    let n: u16 = 0x1234;
    println!("n is: {:04x}", n);
    println!("n is: {:?}", n);

    let big_endian: [u8; 2] = n.to_be_bytes();
    let little_endian: [u8; 2] = n.to_le_bytes();

    println!("n in big endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little endian: {:02X}{:02X}", little_endian[0], little_endian[1]);









}

