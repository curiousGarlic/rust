// FUNCTIONS

// fn main() {
//     println!("___");
//     println!("//functions");
//     another_function(5, 6.9);
// }

// fn another_function(x: i32, y: f32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);

//     let x = 7;

//     let y = {
//         let x = five();
//         x + 1.9
//     };

//     if y < 5.7 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
//     println!("The value of y is: {}", y);
// }

// fn five() -> f32 {
//     5.0
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 { // slow - the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!")
// }
fn main() { // with for loop
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!")
}


// fn main() {
//     let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

//     for element in a.iter() {
//         ;println!("the value is: {}", element);
//     }
// }

