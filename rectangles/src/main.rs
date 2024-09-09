
//example without tuples
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//example with tuples
//prone to errors with someone new introduced into our code
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//example with structs/refactoring with structs BEST ONE
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// //create an instance of a Rectangle struct with values
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 30,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(&rect1)
//     );
// }
// //accesses the width/height fields of the Rectangle instance
// //gives more clarity to the properties of the struct
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// debug example
//this calls the debug functionality provided with Rust to print out debugging information
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30, 
//         height: 50,
//     };

//     //this will call for debug information on rect1
//     println!("rect1 is {rect1:?}");
// }

//better debug example using the dbg! macro
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     //this prevents debug from taking ownership of rect1
//     dbg!(&rect1);
// }

// creating a new macro
// #[derive(Debug)]
// struct  Rectangle {
//     width: u32,
//     height: u32,
// }

// // used for the creation of a method
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// Giving a method the same name as the struct's fields
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }


// methods with multiple parameters
// struct Rectangle{
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
    
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 60, 
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// associated functions
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }
