fn main() {
    let target_number = 69;
    let number = 6;

    if number != 0 {
        println!("might be sexy, its not ZERO at least");
    } else {
        println!("NOT sexy, its ZERO!!!");
    }

    if number < target_number || number > target_number {
        println!("number is NOT sexy!");
    } else {
        println!("number is VERY sexy!");
    }

    if number % 4 == 0 {
        println!("the number is divisible by 4");
    }

    if number % 3 == 0 {
        println!("the number is divisible by 3");
    }

    if number % 2 == 0 {
        println!("the number is divisible by 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 69 };

    println!("The value of number2 is: {number2}");

    let condition2 = false;
    let number3 = if condition2 { 5 } else { "six".len() };

    println!("The value of number3 is: {number3}")

    /*match number {
        number % 4 => println!("the number is divisible by 4"),
    };*/

    /* else {
        println!("number is not divisble by either 4, 3 or 2");
    }*/
}
