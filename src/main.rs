use std::io;

fn main() {

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("incorrect input: {}", e);
        return;
    }

    let numbers: Vec<&str> = input.trim().split_whitespace().collect();

    if numbers.len() != 2 {
        eprintln!("it is not 2 numbers");
        return;
    }

    let a: i64 = match numbers[0].parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("error parsing first number: {}", e);
            return;
        }
    };

    let b: i64 = match numbers[1].parse() {
        Ok(num) => num,
        Err(e) => {
            eprintln!("error parsing second number: {}", e);
            return;
        }
    };

    println!("{}", a + b);
}

