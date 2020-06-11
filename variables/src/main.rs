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
}
