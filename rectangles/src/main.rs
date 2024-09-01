
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

//example with structs/refactoring with structs
struct Rectangle {
    width: u32,
    height: u32,
}

//create an instance of a Rectangle struct with values
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

//accesses the width/height fields of the Rectangle instance
//gives more clarity to the properties of the struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}