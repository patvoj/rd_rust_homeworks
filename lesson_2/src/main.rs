use slug::slugify;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Invalid input.");
        exit(1);
    }

    let op = &args[1];
    let text = &args[2];

    let res = match op.as_str() {
        "lowercase" => text.to_lowercase(),
        "upercase" => text.to_uppercase(),
        "no-spaces" => text.replace(" ", ""),
        "slugify" => slugify(text),
        "reverse" => text.chars().rev().collect(),
        "binary" => text
            .chars()
            .map(|c| format!("{:08b}", c as u8))
            .collect::<Vec<String>>()
            .join(" "),
        _ => {
            eprintln!("Invalid operation: {}", op);
            exit(1);
        }
    };

    println!("{}", res);
}
