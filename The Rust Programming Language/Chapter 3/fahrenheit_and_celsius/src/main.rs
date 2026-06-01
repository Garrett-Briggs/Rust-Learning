use std::io;

fn main() {
    let number = loop {
        println!();
        println!("What is the current temp(only integer)?");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");
        match temperature.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    let which_unit = loop {
        println!();
        println!("What is the unit [C|F]?");
        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        let cleaned_unit = unit.trim().to_uppercase();

        if cleaned_unit == "C" || cleaned_unit == "F" {
            break cleaned_unit
        } else {
            continue;
        }
    };
    match which_unit.as_str() {
        "C" => println!("The temperature in Celsius is {}", (number * 9 / 5) + 32),
        "F" => println!("The temperature in Celsius is {}", (number - 32) * 5 / 9),
        _ => unreachable!(),
    }

}
