use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index: {index} is: {element}.");
}

/*
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    
    x = 6;
    println!("The value of x is: {x}");

    x += 1;
    println!("The value of x is: {x}");

    println!("");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Time, is our greatest enemy: {}", THREE_HOURS_IN_SECONDS);

    println!("");
    println!("------------------------------------------------------");
    println!("");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let tup: (i32, f64, u8) = (500, 6.9, 1);

    println!("x = {}", tup.0);
    println!("y = {}", tup.1);
    println!("z = {}", tup.2);

    let mut tup: (i32, f64, u8) = (500, 6.9, 1);
    let mut (x, y, z) = tup;

    x = 100;
    y = 1.3;
    z = 2;

    println!("x = {x}");
    println!("y = {y}");
    println!("z = {z}");
}
*/
