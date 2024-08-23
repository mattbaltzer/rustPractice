struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@eexample.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//         user1.email = String::from("anotheremail@example.com");
// }

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@eexample.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

        user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username, // shorthand because the struct and funtion have the same names for the key/value
        email, // shorthand because the struct and funtion have the same names for the key/value
        sign_in_count: 1,
    }
}
