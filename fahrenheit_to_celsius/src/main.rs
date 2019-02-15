use std::io;

fn main() {
    loop {
        match read_conversion_type() {
            Ok(0) => {
                println!("Converting Celsius to Fahrenheit");
                match read_value_to_convert() {
                    Ok(value_to_convert) => println!(
                        "Result: {}°C -> {}°F",
                        value_to_convert,
                        to_fahrenheit(value_to_convert)
                    ),
                    Err(error) => {
                        println!("{}", error);
                        break;
                    }
                }
            }
            Ok(1) => {
                println!("Converting Fahrenheit to Celsius");
                match read_value_to_convert() {
                    Ok(value_to_convert) => println!(
                        "Result: {}°F -> {}°C",
                        value_to_convert,
                        to_celsius(value_to_convert)
                    ),
                    Err(error) => {
                        println!("{}", error);
                        break;
                    }
                }
            }
            Ok(_) => {
                println!("Bye");
                return;
            }
            Err(error) => {
                println!("{}", error);
                return;
            }
        }
    }
}

fn read_conversion_type() -> Result<u8, &'static str> {
    println!("Select conversion type:");
    println!("[0] °C -> °F");
    println!("[1] °F -> °C");
    println!("[>1] Exit");
    let mut conversion_type: String = String::new();
    io::stdin()
        .read_line(&mut conversion_type)
        .expect("Fatal error while reading conversion type.");
    match conversion_type.trim().parse() {
        Ok(conversion_type) => Ok(conversion_type),
        Err(_) => Err("Illegal conversion type."),
    }
}

fn read_value_to_convert() -> Result<f32, &'static str> {
    println!("Insert value to convert:");
    let mut value_to_convert: String = String::new();
    io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Fatal error while reading value to convert.");
    match value_to_convert.trim().parse() {
        Ok(value_to_convert) => Ok(value_to_convert),
        Err(_) => Err("Illegal value to convert."),
    }
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 9.0 / 5.0 + 32.0
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

#[cfg(test)]
mod tests {

    use super::to_celsius;
    use super::to_fahrenheit;

    #[test]
    fn celsius_to_fahrenheit() {
        assert_eq!(50.0, to_fahrenheit(10.0));
    }

    #[test]
    fn fahrenheit_to_celsius() {
        assert_eq!(10.0, to_celsius(50.0));
    }

}
