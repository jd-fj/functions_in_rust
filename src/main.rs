// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }
// --------------------------------------------------

// * Parameters *

// fn main() {
//     another_function(5);
// }

// // In function signatures, you must declare the type of each parameter
// fn another_function(x: i8) {
//     println!("The value of x is: {}", x);
// }
// --------------------------------------------------

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i8, unit_label: char) {
//     println!("the measurement is: {}{}", value, unit_label);
// }
// --------------------------------------------------

//  * EXPRESSIONS VS. STATEMENTS * 
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Expressions can be part of statements

// fn main() {
//     let y = 6; // this a statement
// }

// everything in the curly braces after 'let y =' is an expression. Note that the x + 1 line doesn’t have a semicolon at the end. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }
// --------------------------------------------------

//  * FUNCTIONS WITH RETURN VALUES * 
// We don't name return values, but we must declare their type after an arrow.

// fn five() -> i8 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }
// --------------------------------------------------

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
