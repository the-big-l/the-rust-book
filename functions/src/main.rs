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

    fn fibonacci(n: u8) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    let n = 10;
    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
    
    const ORDINALS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];
        
    const GIFTS: [&str; 12] = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // Day 1: just add the first gift: gifts[0]
    // Day 2: add the second gift, add " and ", add the first gift
    // Day 2+: build gift line, add ", and ", add the first gift
    fn sing_twelve_days_of_christmas() {
        for day in 0..12 {
            let ordinal = ORDINALS[day];
            let mut verse = format!("On the {} day of Christmas my true love gave to me ", ordinal);
            let gift_line = build_gift_line(day);
            
            verse = match day {
                0 => verse + GIFTS[0],
                1 => verse + GIFTS[1] + " and " + GIFTS[0],
                _ => verse + &gift_line + ", and " + GIFTS[0],
            };
            
            println!("{}.\n", verse);
        }
    }

    // Adds all gifts for "day" except the first gift, concat with ", "
    fn build_gift_line(day: usize) -> String {
        GIFTS[1..=day].iter()
            .rev()                  // reverse the order
            .map(|s| *s)            // dereference &str to str
            .collect::<Vec<&str>>() // collect into a vector of &str
            .join(", ")             // join with ", "
    }

    sing_twelve_days_of_christmas();
}