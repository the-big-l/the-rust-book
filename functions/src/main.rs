fn main() {
    println!("Hello, world!");

    print_lable_measurement(5, 'm');

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn print_lable_measurement(value: i32, lable: char) {
    println!("The measurement is: {}{}", value, lable);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}