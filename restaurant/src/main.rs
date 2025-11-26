// use restaurant::eat_at_restaurant2;

mod pizza {
    pub fn kurac() {}
}

fn main() {
    use restaurant::eat_at_restaurant2;
    eat_at_restaurant2();
    
    pizza::kurac();

    restaurant::hosting::add_to_waitlist();
}
