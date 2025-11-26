fn main() {
    let mut s1 = String::from("Zlatan Omerović");

    let s2 = String::from("Zlatan Omerović");
    take_ownership(s2, &mut s1);

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut x = String::from("Zlatan");
    change(&mut x);
    println!("x = {}", x);

    // let mut p = String::from("Zlatan");
    // let r1 = &mut p;
    // let r2 = &mut r1;
    //
    // println!("r1 - {r2}");

    // let s = String::from("hello");

    // let r12 = &s; // no problem
    // let r22 = &s; // no problem
    // // let r32 = &mut s; // BIG PROBLEM
    //
    // println!("{r12}, {r22}, and {r32}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    // let s = String::from("hello");
    // let reference_to_nothing = dangle();
}

// fn dangle() -> String {
//     let s = String::from("hello");
//     s
//     // x
//     // &s
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn take_ownership(mut s: String, s1: &mut String) {
    s.push_str(" Hola");
    println!("{} + {}", s, s1.as_str());
    s1.push_str(" TITO");
    // *s1 = String::from("asdf");
    println!("x = {}", s1.as_str());
}

fn change(some_string: &mut String) {
    some_string.push_str("Hola");
}