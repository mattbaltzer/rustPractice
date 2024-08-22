// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");
// }

// fn main() {                      // s is not valid here, it’s not yet declared
//     let s = "hello";            // s is valid from this point forward

//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid


// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{s}"); // This will print 'hello, world!'
// }


// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s); // s' value moves into the function and is no longer valid here

//     let x = 5;  // x comes into scope

//     makes_copy(x);  // x moves into the function, but i32 is Copy so it can be used after

// }   // x goes out of scope, then s. s' value was moved so nothing special happens

// fn takes_ownership(some_string: String) {   // some_string comes into scope
//     println!("{some_string}");
// }   // Here some_string goes out of scope and `drop` is called. Memory is freed

// fn makes_copy(some_integer: i32) {  // some_integer comes into scope
//     println!("{some_integer}");
// }   // Here some_integer goes out of scope. Nothing special happens

// fn main() {
//     // gives_ownership moves its return value into s1
//     let s1 = gives_ownership();

//     // s2 comes into scope
//     let s2 = String::from("hello");

//     // s2 is moved into takes_and_gives_back, which also moves
//     // its return value into s3
//     let s3 = takes_and_gives_back(s2);
// } // Here, s3 goes out of scope and is dropped. s2 was moved
// // so nothing happens. s1 goes out of scope and is dropped

// // gives_ownership will move its return value into the function
// // that calls it
// fn gives_ownership() -> String {

//     // some_string comes into scope
//     let some_string = String::from("yours");

//     // some_string is returned and moves out to the calling function
//     some_string
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into scope

//     // a_string is returned and moves out to the calling function
//     a_string
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     // len() returns the length of a string
//     let length = s.len(); 

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// // s is a reference to a String
// fn calculate_length(s: &String) -> usize {
//     s.len()
// } // Here s goes out of scope. Because it does not have ownership of what it refers to
// // it is not dropped

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}", r1, r2, r3);
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn main() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
}