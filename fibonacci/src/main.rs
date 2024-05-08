use std::io;

fn main() {
    println!("Enter desired nth Fibonacci number: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u64 = n.trim().parse().expect("Please enter a non-negative integer.");

    let nth_fib: u64 = if n == 0 {
        0
    } else {
        let mut n_m_2: u64;
        let mut n_m_1: u64 = 0;
        let mut n_value: u64 = 1;

        for _ in 1..n {
            n_m_2 = n_m_1;
            n_m_1 = n_value;
            n_value = n_m_2 + n_m_1;
        }

        n_value
    };

    println!("The {n}{} Fibonacci number is: {nth_fib}", ordinal_suffix(n));
}

fn ordinal_suffix(n: u64) -> String {
    let last_two_digits = n % 100;
    let last_digit = n % 10;

    if last_two_digits >= 10 && last_two_digits <= 19 {
        "th".to_string()
    } else if last_digit == 1 {
        "st".to_string()
    } else if last_digit == 2 {
        "nd".to_string()
    } else if last_digit == 3 {
        "rd".to_string()
    } else {
        "th".to_string()
    }
}
