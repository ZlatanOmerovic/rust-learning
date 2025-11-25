fn main() {
    println!("Hello, world!");

    another_function(69, 'X');

    let sixty_nine = i_love_sixty_nine(69);

    println!("sixty nine: {sixty_nine}.");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");

    let y = {
        let x = 100;
        x * 10
    };

    println!("The value of y is: {y}");
}

fn i_love_sixty_nine(int: i32) -> i32 {
    int * 3
}