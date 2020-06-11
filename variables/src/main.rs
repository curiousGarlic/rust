// VARIABLES

fn main() {
    // println!("___");
    // println!("//let and mut let and const");

    // const MAX_NUMBER: u32 = 1_000_000;
    // println!("Max const number is {}", MAX_NUMBER);

    // let mut x = 5;
    // println!("The value of x is {}", x);

    // x = 7;
    // println!("The value of x became {} now", x);

    println!("___");
    println!("//shadowing");
    let x = 2;
    let x = x + 3;
    let x = x * 5;
    println!("Shadowed x:    {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("# of spaces:    {}", spaces);

    println!("___");
    println!("//floating-point numbers");

    let z = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("z -> {} and y -> {}", z, y);

    println!("___");
    println!("//compound types - tuples");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Value of tup index 0 is: {}", tup.0);
    println!("The value of y is: {}", y);

    println!("___");
    println!("//compound types - arrays");
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("First month:    {}", months[0]);
    
    let a: [i32; 5] = [1,2,3,4,5];
    println!("Last array item:    {}", a[4]);

    println!("___");
    println!("//functions");
}
