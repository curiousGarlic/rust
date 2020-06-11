// FUNCTIONS

fn main() {
    println!("___");
//     println!("//functions");
//     another_function(5, 6.9);
// }

// fn another_function(x: i32, y: f32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}