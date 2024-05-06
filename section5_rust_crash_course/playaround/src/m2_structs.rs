#[derive(Debug)] // this has to be added to the struct for it to be able to use debug
struct User { // this struc is a type. It holds user information
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) { // self refers to the instance of the struct above
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) { // if you only want to read the value of the variable, you can use &self
        self.email = String::from(new_email);
    }
}

fn change_username(user: &mut User, new_username: &str) { // this function changes the username
    // &mut is a mutable reference. It allows us to change the value of the variable
    // & is a reference. It allows us to pass the variable by reference
    // &str is a string slice. It is a reference to a string
    user.username = String::from(new_username); // this changes the username
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let mut user1: User = User {
            username: String::from("someusername123"),
            email: String::from("some@example.com"),
            active: true,
            sign_in_count: 1,
        }; // creating a function that does the same thing for us as below
        // //user1.username = String::from("anotherusername456");
        // user1.username = "anotherusrname".to_string();
        change_username(&mut user1, "anotherusername456"); // why would we keep passing the function for every user. instead we can create impl
        dbg!(&user1);

        let mut user_2: User = User {
            username: String::from("anotherusername2"),
            email: String::from("some@example2.com"),
            active: false,
            sign_in_count: 50,
        };
        user_2.increment_sign_in_count();
        user_2.change_email("anotheremail@email.com");
        dbg!(&user_2);
    }
}