#[derive(Debug)]
struct Test {
    name: String,
    count: i32,
}

fn main() {
    println!("Hello, collections!");

    let v: Vec<i32> = Vec::new();

    let mut vv: Vec<Test> = Vec::new();

    let mut test1 = Test {
        name: String::from("tester1"),
        count: 69,
    };
    /*
    let test2 = &mut test1;
    test2.name = String::from("tester2");
    test2.count += 1;
     */
    let test2 = Test {
        // ..&test1
        name: String::from("tester2"),
        count: 70,
    };

    // let test: Option<&Test> = &test1;

    vv.push(test1);
    vv.push(test2);

    dbg!(&vv);

    let v = vec![1, 2, 3];
    dbg!(&v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    dbg!(&v[0]);
    dbg!(&v[1]);
    dbg!(&v[2]);
    dbg!(&v[3]);

    println!("v.len() = {}", v.len());
    println!("v.capacity = {}", v.capacity());
    println!("v.iter.count() = {}", vv.iter().count());
    println!("v.iter.len() = {}", vv.iter().len());

    // dbg!(&v[4]);

    let mut v = &mut vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The FOURTH element is {fourth}"),
        None => println!("There is no FOURTH element."),
    }

    /*
    let mut third: &i32 = &mut &v[2];
    println!("The third element is {}", third);
    dbg!(&v);
    let thirty_three = 33;
    third = &thirty_three;
    println!("The third element is {}", third);
    dbg!(&v);
     */

    let v = vec![1, 2, 3, 4, 5];

    let mut index = 1;
    let ref_index: &i32 = &index;

    let does_exist = &v[*ref_index as usize];
    dbg!(does_exist);

    index += 100;
    let does_not_exist = v.get(index as usize);
    dbg!(does_not_exist);

    let test = match does_not_exist {
        Some(value) => value,
        None => &-1,
    };

    if (*test == -1) {
        println!("No value found at index: {}", index)
    }

    // ------------------------------------------

    let mut v = vec![1, 2, 3, 4, 5];
    //

    v.push(6);
    v.push(7);
    v.push(8);

    let first = &v[0];

    println!("The first element is: {first}");

    dbg!(&v);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    dbg!(v);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    dbg!(v);
}
