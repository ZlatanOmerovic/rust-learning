struct UserRole {
    active: bool,
    name: &'static str,
}

struct User {
    id: i32,
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    role: UserRole,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, structs!");

    // let subject = AlwaysEqual;

    let mut id: i32 = 0;
    let mut user1 = User {
        id: { id += 1; id },
        active: true,
        username: String::from("zlomerovic"),
        email: String::from("zlomerovic@hotmail.com"),
        sign_in_count: 69,
        role: UserRole {
            active: true,
            name: "Administrators"
        }
    };

    user1.email = "ascent.ba@gmail.com".to_string();
    user1.role.name = "Testers";

    let user2 = User {
        id: { id += 1; id },
        active: true,
        username: String::from("zlomerovic"),
        email: String::from("zlomerovic@hotmail.com"),
        sign_in_count: 69,
        role: UserRole {
            active: true,
            name: "Administrators"
        }
    };

    print_user(&user1);
    print_user(&user2);

    let user3 = build_user(String::from("kubat"), String::from("mahir.omerovic@osa-oba.gov.ba"), &mut id);
    print_user(&user3);

    let user4 = User {
        id: { id += 1; id },
        email: String::from("mahir.omerovic@gmail.com"),
        ..user3
    };
    print_user(&user4);

    let user5 = build_user(String::from("bećir denisović"), String::from("becir@predsjednistvo.gov.ba"), &mut id);
    print_user(&user5);

    let lime = Color(128, 255, 0);
    let origin = Point(800, 600, 480);

    print_color(lime);
    print_point(origin);

    let orange = Color(255, 128, 0);
    let origin = Point(1920, 1024, 768);

    print_color2(orange);
    print_point2(origin);
}

fn print_color(color: Color) {
    println!("red   = {}", color.0);
    println!("green = {}", color.1);
    println!("blue  = {}", color.2);
    println!();
}

fn print_point(point: Point) {
    println!("x = {}", point.0);
    println!("y = {}", point.1);
    println!("z = {}", point.2);
    println!();
}

fn print_color2(color: Color) {
    let Color(red, green, blue) = color;
    println!("red   = {}", red);
    println!("green = {}", green);
    println!("blue  = {}", blue);
    println!();
}

fn print_point2(point: Point) {
    let Point(x, y, z) = point;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!();
}

fn print_user(user: &User) {
    println!("id: {}", user.id);
    println!("active: {}", user.active);
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("sign_in_count: {}", user.sign_in_count);
    println!("role.active: {}", user.role.active);
    println!("role.name: {}", user.role.name);
    println!();
}

fn build_user(username: String, email: String, id: &mut i32) -> User {
    User {
        id: { *id += 1; *id },
        active: true,
        username,
        email,
        sign_in_count: 1,
        role: UserRole {
            active: true,
            name: "Administrator"
        },
    }
}