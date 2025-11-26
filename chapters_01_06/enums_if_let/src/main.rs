// use crate::Coin::Quarter;

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

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::NewYork => year >= 1696,
        }
    }
}

fn main() {
    //let config_max = Some(3u8);
    let config_max = Some(69);
    if let Some(max) = config_max && max == 69 {
        println!("The maximum is configured to be {max}");
    } else {
        println!("config_max = {}", config_max.unwrap())
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::NewYork);
    dbg!(&coin);
    if let Coin::Quarter(state) = coin {
        //state = UsState::Alabama;
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    dbg!(count);

    println!("----------------------------------------");

    debug_descriptors();
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

fn describe_state_quarter2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter3(coin: Coin, year: u16) -> Option<String> {
    let Coin::Quarter(state): Coin = coin else {
        return None;
    };

    if state.existed_in(year) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn debug_descriptors() {
    let coin1 = Coin::Quarter(UsState::Alaska);
    let coin2 = Coin::Quarter(UsState::Alabama);
    let coin3 = Coin::Quarter(UsState::NewYork);

    let produce1 = describe_state_quarter(coin1);
    let produce2 = describe_state_quarter2(coin2);
    let produce3 = describe_state_quarter3(coin3, 18);

    let a = match &produce1 {
        Some(a) => a,
        None => &String::from("test")
    };

    dbg!(&produce1);
    dbg!(&produce2);
    dbg!(&produce3);

    dbg!(&a);
    println!("a = {}", &a);

    let b = match produce3 {
        Some(a) => a,
        None => String::from("test")
    };

    dbg!(&b);
    println!("b = {}", &b);
}
