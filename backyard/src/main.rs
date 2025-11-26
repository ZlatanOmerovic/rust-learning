use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    garden::zlatan::zlatan(String::from("Mehmedizacija Bez Presedana!"));

    let a = garden::Test {
        best: 69,
    };

    let b = garden::Test::factory(100);
    b.debug();

    garden::Test::debug2(a);
    garden::Test::debug2(b);
}
