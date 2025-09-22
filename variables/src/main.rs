fn main() {
    let x = "xXxXxX";
    let x_type = std::any::type_name_of_val(&x);

    
    {
        let x: u8 = x.len().try_into().unwrap();
        let x_type = std::any::type_name_of_val(&x);

        let y = 555_u32.wrapping_add(1);
        // let yy = 555_u8.wrapping_add(u8::MAX);
        let z: u8 = 555.try_into().unwrap_or(u8::MAX);

        println!("y = {y}");
        // println!("yy = {yy}");
        println!("z = {z}");

        println!("The type of x in the inner scope is: {}", x_type);
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value type of x is: {}", x_type);
    println!("The value of x is: {x}");
}