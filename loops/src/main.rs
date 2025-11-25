fn main() {
    println!("Hello, world!");

    let mut index: i32 = 0;
    const MAX: i32 = 10;

    loop {
        println!("{} - again!", index);
        index += index + 1;

        if index >= MAX {
            break;
        }
    };

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == MAX {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!("-------------------------------------------------------");

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }

            if count == 10 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
        println!("");
    }

    println!("End count = {count}");

    println!("-------------------------------------------------------");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("-------------------------------------------------------");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 1000 {
        if index >= a.len() {
            println!("unreachable index: {}, there are only {} items inside a-array", index, a.len());
            break;
        }

        let value = a[index];

        if index == 3 {
            println!("we don't like index = 3, with value of {}, we'll skip it, won't print it...", value);
            index += 1;
            continue;
        }
        
        println!("the value is: {}", value);

        index += 1;
    }

    println!("LIFTOFF again!!!");

    println!("-------------------------------------------------------");

    let b = [10, 20, 30, 40, 50, 69];
    // let _c = [10, 20, 30, 40, 50, 69];

    for element in b {
        println!("the value is: {element}");
    }

    println!("third LIFTOFF!!!");

    println!("-------------------------------------------------------");

    for number in (1..4).rev() {
        println!("number: {number}!");
    }

    println!("4th LIFTOFF!!!");

    println!("-------------------------------------------------------");

    let temperature = 69.13;
    let fahr_to_cels = fahrenheit_to_celsius(temperature);
    println!("{} fahrenheit in celsius is: {}", temperature, fahr_to_cels);
    println!("");
    let cels_to_fahr = celsius_to_fahrenheit(temperature);
    println!("{} celsius in fahrenheit is: {}", temperature, cels_to_fahr);
    println!("");

    println!("end of temperature aptitute test");

    println!("-------------------------------------------------------");

    let fib_num: i32 = 10;
    let fibonacci_result = fibonacci(fib_num);
    println!("fibonacci_result = {fibonacci_result}");

    let fib_num2: i32 = 7;
    let fibonacci_result2 = fibonacci(fib_num2);
    println!("fibonacci_result2 = {fibonacci_result2}");

    let fibonacci_recursive = fib(fib_num);
    println!("fibonacci_recursive = {fibonacci_recursive}");
    let fibonacci_recursive2 = fib(fib_num2);
    println!("fibonacci_recursive2 = {fibonacci_recursive2}");

    let mut memo = std::collections::HashMap::new();
    let mut memo2 = std::collections::HashMap::new();

    let fib_memo = fibonacci_memo(fib_num, &mut memo);
    println!("fib_memo = {fib_memo}");
    let fib_memo2 = fibonacci_memo(fib_num2, &mut memo2);
    println!("fib_memo2 = {fib_memo2}");

    println!("-------------------------------------------------------");

    song();
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}

fn fibonacci(n: i32) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn fib(n: i32) -> u64 {
    if n <= 1 { return n as u64; }
    return fib(n - 1) + fib(n - 2);
}

fn fibonacci_memo(n: i32, memo: &mut std::collections::HashMap<i32, u64>) -> u64 {
    if let Some(value) = memo.get(&n) {
        return *value;
    }

    if n <= 1 {
        return n as u64;
    }

    let result = fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo);
    memo.insert(n, result);

    result
}

fn song() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("my true love sent to me");

        for i in (0..=day).rev() {
            // For days after the first, prefix the last line with "And "
            if day > 0 && i == 0 {
                println!("And {}", gifts[i]);
            } else {
                println!("{}", gifts[i]);
            }
        }

        println!(); // blank line between verses
    }
}
