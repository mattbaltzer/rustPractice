fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

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