#[derive(Debug)] // this has to be added to the struct for it to be able to use debug
struct  User { // this struc is a type. It holds user information
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let user1: User = User {
            username: String::from("someusername123"),
            email: String::from("some@example.com"),
            active: true,
            sign_in_count: 1
        };

        dbg!(&user1);
        }
    }
