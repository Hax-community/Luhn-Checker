use std::{io::{Read, self, Write}, fs::File, path::Path};

use argparse::{ArgumentParser, Store};

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn write(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

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
    let mut valid_cards: String = String::new();
    let mut fname = "cards.txt".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Cards checker that uses luhn algorithm");
        ap.refer(&mut fname)
            .add_option(&["-f", "--fname"], Store,
            "Name of file for checking");
        ap.parse_args_or_exit();
    }

    let file_check_contain = match cat(&Path::new(&fname)) {
        Ok(s) => s,
        Err(e) => panic!("Error {}", e),
    };

    for i in file_check_contain.split("\n"){
        if luhn_checksum(i) {
            valid_cards.push_str(i);
        };
    }

    let _ = write(&valid_cards, &Path::new("valids.txt"));

    //let res = luhn_checksum(input);
}
