use std::io;

fn luhn_checksum(input: &str) -> bool {
    let mut sum = 0;
    let mut parity = input.len() % 2 == 0;

    for c in input.chars().rev() {
        if let Some(mut digit) = c.to_digit(10) {
            if parity {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            sum += digit;
            parity = !parity;
        }
    }
    sum % 10 == 0
}
fn main() {
    let mut _input: String = String::new();

    println!("enter card n: ");
    io::stdin().read_line(&mut _input).expect("error while reading line");

    let input = _input.trim();
    let res = luhn_checksum(input);
    println!("valid? {}", res);
}
