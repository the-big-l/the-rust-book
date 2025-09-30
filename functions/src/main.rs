fn main() {
    // print Hello, world!
    println!("Hello, world!");

    print_lable_measurement(5, 'm');

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
    
    // This function prints a measurement with its label. The label is a character.
    // For example, 'm' for meters, 's' for seconds, etc.
    fn print_lable_measurement(value: i32, lable: char) {
        println!("The measurement is: {}{}", value, lable);
    }
    
    fn five() -> i32 {
        5
    }
    
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    
    // Function to convert Celsius to Fahrenheit
    fn celcius_to_fahrenheit(c: f64) -> f64 {
        (c * 9.0/5.0) + 32.0
    }
    
    let temp_c = 25.0;
    let temp_f = celcius_to_fahrenheit(temp_c);
    println!("{}°C is {}°F", temp_c, temp_f);

    // Generic function to convert temperatures
    fn convert_temp(value: String) {
        let temp: f64 = value.chars().take(value.len() - 1).collect::<String>().parse().unwrap();
        let unit: char = value.chars().last().unwrap();

        // match unit {
        //     'C' => {
        //         let fahrenheit = (temp * 9.0/5.0) + 32.0;
        //         println!("{}°C is {}°F", temp, fahrenheit);
        //     },
        //     'F' => {
        //         let celsius = (temp - 32.0) * 5.0/9.0;
        //         println!("{}°F is {}°C", temp, celsius);
        //     },
        //     _ => println!("Unknown unit"),
        // }

        if unit == 'C' {
            let fahrenheit = (temp * 9.0/5.0) + 32.0;
            println!("{}°C is {}°F", temp, fahrenheit);
        } else if unit == 'F' {
            let celsius = (temp - 32.0) * 5.0/9.0;
            println!("{}°F is {}°C", temp, celsius);
        } else {
            println!("Unknown unit");
        }
    }

    convert_temp("100C".to_string());
    convert_temp("37C".to_string());
    convert_temp("212F".to_string());
    convert_temp("98.6F".to_string());
}