use std::io;

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x: {}, y: {}", x, y);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}", sum, difference, product, quotient, truncated, remainder);
    
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t: {}, f: {}", t, f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = arr[1];
    
    println!("first: {}, second: {}", first, second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);

}
