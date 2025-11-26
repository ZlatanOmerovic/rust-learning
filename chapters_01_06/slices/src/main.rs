fn main() {
    println!("Hello, world!");

    let s = String::from("zlatan omerovic");

    let word = first_word(&s);

    println!("word = {} = {}", s, word);
    // s.clear();
    // println!("word = {} = {}", s, word);

    let my_string = String::from("hello world");

    let word = first_word(&my_string[6..10]);
    println!("word = {} = {}", my_string, word);
    let word = first_word(&my_string[7..]);
    println!("word = {} = {}", my_string, word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!("word = {} = {}", my_string, word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("word = {} = {}", my_string_literal, word);
    let word = first_word(&my_string_literal[..]);
    println!("word = {} = {}", my_string_literal, word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word = {} = {}", my_string_literal, word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[2..5];

    assert_eq!(slice, &[3, 4, 5]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}