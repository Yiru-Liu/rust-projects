use std::io;

fn main() {
    println!("Input temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = temp.trim().parse().expect("Please enter a number.");

    println!("Input unit (C, F, K): ");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    let unit: char = unit.trim().parse().expect("Please enter C, F, or K.");

    println!("Input target unit (C, F, K): ");
    let mut target_unit = String::new();
    io::stdin()
        .read_line(&mut target_unit)
        .expect("Failed to read line");
    let target_unit: char = target_unit.trim().parse().expect("Please enter C, F, or K.");

    let temp_celsius = if unit == 'C' {
        temp
    } else if unit == 'F' {
        (temp - 32.0) * 5.0 / 9.0
    } else if unit == 'K' {
        temp - 273.15
    } else {
        panic!("Unrecognized unit: {unit}")
    };

    let output_temp = if target_unit == 'C' {
        temp_celsius
    } else if target_unit == 'F' {
        temp_celsius * 9.0 / 5.0 + 32.0
    } else if target_unit == 'K' {
        temp_celsius + 273.15
    } else {
        panic!("Unrecognized unit: {target_unit}")
    };

    println!("{temp} {unit} = {output_temp} {target_unit}")
}
