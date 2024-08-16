// Accepts user inputs and print the result as output we bring in the (io) input/output library
// (io) comes from the standard librarby (std)
// To pull this into scope, you need to declare it with use, since it isn't in the prelude
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // gen_range method is brought into scope with the use rand::Rng statement. gen_range is an expression that takes start..=end values and is inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // loop creates an infinite loop
    loop {
        println!("Please input your guess.");

        // Creates a mutable(it can change) variable called guess, sets the value of guess to a new String type
        // The :: indicates that new is an associated function of the String type
        // Associated functions are functions that are implemented on a type, in this case String
        let mut guess = String::new();

        // This calls the stdin function from the io module, which allows us to handle user input
        io::stdin()
            // This calls the read_line method on the reference of the mutable guess variable
            .read_line(&mut guess)
            // This is basically OK and Err handling, basically it will crash the program and display the message in the expect
            .expect("Failed to read the line");
        // trim() will eliminate any whitespace on a string. parse will convert a string to another type, we're setting it to a u32 value
        // Created values for error handling, numbers are okay and anything else will just continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Think of the {} as pincers, they hold the value in place
        println!("You guessed: {guess}");

        // Checks the values of guess against secret_number, it then runs through each arm to see what the values are
        // If guess is greater than secret_number, it will print out "Too big!"
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
