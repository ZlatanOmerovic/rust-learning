use crate::Coin::Quarter;

#[derive(Debug)]
enum UsState {
    NewYork,
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    let config_max = Some(69);
    if let Some(max) = config_max && max == 69 {
        println!("The maximum is configured to be {max}");
    } else {
        println!("config_max = {}", config_max.unwrap())
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::NewYork);
    dbg!(&coin);
    if let Coin::Quarter(mut state) = coin {
        state = UsState::Alabama;
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    dbg!(count);
}
