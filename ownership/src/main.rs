fn main() {
    let s = String::from("hello world");

    {
        let s = String::from("hello world 222");
        println!("{}", s);
    }

    let s1 = String::from("helloXXX");
    let s2 = s1;

    println!("{}", s2);

    let mut x = String::from("hello world");
    println!("x = {}", x);
    x = String::from("merhaba");
    println!("x = {}, dunjaluče! 69", x);

    println!("\n\n\n");

    println!("{}", s);

    let xxx = String::from("hello 69th world");
    // takes_ownership(xxx);
    // let ko = xxx.clone();
    println!("{}", xxx);
    let t: String = takes_ownership2(xxx);
    println!("{}", t);

    let yyy = 5;
    makes_copy(yyy);
    println!("yyy = {}", yyy);

    // --------------------------------

    // let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    // --------------------

    let zlatan = String::from("Zlatan Omerovic aka. Testla");
    let (len, str) = calculate_length(zlatan);

    println!("The length of '{}' is {}.", str, len);
}

fn calculate_length(s: String) -> (usize, String) {
    // let len = s.len();
    (s.len(), s)
}

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn _takes_ownership(_some_string: String) {
    println!("{}", _some_string);
    // _some_string = String::from("hello world");
    // mut some_string: String :::
    // some_string.push_str(" pička");
    // println!("{}", some_string);
}

fn takes_ownership2(some_string: String) -> String {
    println!("{}", some_string);
    // some_string = String::from("hello world string value");
    // mut some_string: String :::
    // some_string.push_str(" pička");
    // println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    println!("{}", some_integer + 64);
}

// fn main() {
//     println!("Hello, world!");
//
//     // let s = "hello";
//     //
//     // {
//     //     let s = "hello";
//     // }
//
//     let mut a = "kurac";
//     println!("{}", a);
//     a = concat!("haha", "a");
//
//     let mut s = String::from(a);
//     s.push_str(", world!");
//
//     println!("{}", s);
//     println!("{}", a);
// }
