fn main() {
    let message = "Hello World";
    if message.len() > 10 {
        println!("Message is too long");
    } else {
        println!("Message is OK");
    }

    // data type
    let a: i8 = 10;
    let b: i16 = 20;
    let c: i32 = 30;
    let d: i64 = 40;
    let e: i128 = 50;
    let f: isize = 60;
    let g: u8 = 70;
    let h: u16 = 80;
    let i: u32 = 90;
    let j: u64 = 100;
    let k: u128 = 110;
    let l: usize = 120;
    let m: f32 = 130.0;
    let n: f64 = 140.0;
    let o: bool = true;
    let p: char = 'a';
    let q: &str = "Hello World";
    let r: String = String::from("Hello World");

    // array
    let s = [1, 2, 3, 4, 5];

    // vector
    let t = vec![1, 2, 3, 4, 5];

    // tuple
    let u = (1, 2, 3, 4, 5);

    // struct
    struct Person {
        name: String,
        age: u8,
    }

    let v = Person {
        name: String::from("John"),
        age: 20,
    };
    println!("name: {}, age: {}", v.name, v.age);

    // enum
    enum Color {
        Red,
        Green,
        Blue,
    }

    let w = Color::Red;
    match w {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    // function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let x = add(1, 2);
    println!("x: {}", x);

    // closure
    let y = |a: i32, b: i32| -> i32 { a + b };
    let z = y(1, 2);
    println!("z: {}", z);

    // loop
    let mut aa = 0;
    loop {
        aa += 1;
        if aa > 10 {
            println!("aa: {}", aa);
            break;
        }
    }

    // while
    let mut bb = 0;
    while bb < 10 {
        println!("bb: {}", bb);
        bb += 1;
    }

    // for
    for cc in 0..10 {
        println!("{}", cc);
    }

    // match
    let dd = 1;
    match dd {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("other"),
    }

    // if let
    let ee = 1;
    if let 1 = ee {
        println!("1");
    } else {
        println!("other");
    }

    // while let
    let mut ff = 0;
    while let 10 = ff {
        ff += 1;
    }

    // for let
    let gg = vec![1, 2, 3, 4, 5];
    for i in gg {
        println!("{}", i);
    }

    // option
    let hh = Some(1);
    match hh {
        Some(1) => println!("1"),
        Some(2) => println!("2"),
        _ => println!("other"),
    }

    // result
    let ii: Result<i32, E> = Ok(1);
    // assert_eq
    assert_eq!(1, 1);

    // assert_ne
    assert_ne!(1, 2);

    // debug_assert
    debug_assert!(true);

    // debug_assert_eq
    debug_assert_eq!(1, 1);

    // debug_assert_ne
    debug_assert_ne!(1, 2);

    // todo
    // todo!("todo");

    // unimplemented
    // unimplemented!("unimplemented");

    // unsafe
    // unsafe {
        // unsafe code
    // }

    // extern
    // extern "C" {
        // extern code
    // }


}
