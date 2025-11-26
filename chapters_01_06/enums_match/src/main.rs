#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    California,
    Texas,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Sixty,
}

#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Sixty => {
            println!("I am quarter!");
            23 * 3
        }
    }
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            dbg!(&state);
            println!("State quarter from {state:#?}!");

            100 / 4
        },
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}

fn plus_one2(x: Option<i32>) -> i32 {
    match x {
        _ => 77,
    }
}

fn plus_one3(x: Option<i32>) -> Option<i32> {
    // None
    Some(100)
}

fn plus_one4(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => todo!()
    }
}

fn main() {
    println!("Hello, world! 999");

    plus_one4(None);

    let x = value_in_cents(Coin::Sixty);
    dbg!(x);

    let y = value_in_cents2(Coin2::Quarter(UsState::NewYork));
    dbg!(y);

    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(Some(6));
    let none = plus_one(None);

    let ten = plus_one2(None);

    dbg!(five);
    dbg!(six);
    dbg!(seven);
    dbg!(none);
    dbg!(ten);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // _ => reroll(_),
        // _ => move_player(0),
        // other => move_player(other),
    }
}

fn reroll(i: i32) -> i32 { i }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
