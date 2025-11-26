#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "1 - The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect2 = (30, 50);

    println!(
        "2 - The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "3 - The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    println!("rect1 is {rect3:?}");
    println!("rect1 is {rect3:#?}");

    dbg!(width1, height1);
    dbg!(rect2);
    dbg!(&rect2);
    dbg!(&rect3);

    let scale = 2;
    dbg!(&scale);
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
