use slug::slugify;
use std::error::Error;
use std::io;
use std::sync::mpsc;
use std::thread;

struct Command {
    operation: String,
    text: String,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            input.clear();
            if let Err(e) = stdin.read_line(&mut input) {
                eprintln!("Failed to read input: {}", e);
                break;
            }

            let args: Vec<String> = input.split_whitespace().map(String::from).collect();
            if args.is_empty() {
                continue;
            }

            if args.len() < 2 {
                eprintln!("Usage: <operation> <text>");
                continue;
            }

            let command = Command {
                operation: args[0].clone(),
                text: args[1..].join(" "),
            };

            if let Err(e) = tx.send(command) {
                eprintln!("Failed to send a command: {}", e);
                break;
            };
        }
    });

    while let Ok(command) = rx.recv() {
        let res = match command.operation.as_str() {
            "lowercase" => to_lower_case(&command.text),
            "uppercase" => to_upper_case(&command.text),
            "no-spaces" => remove_spaces(&command.text),
            "slugify" => make_slugify(&command.text),
            "reverse" => reverse_string(&command.text),
            "binary" => to_binary(&command.text),
            "csv" => csv(&command.text),
            _ => {
                eprintln!("Invalid operation: {}", command.operation);
                continue;
            }
        };

        match res {
            Ok(output) => println!("{}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    handle.join().unwrap()
}

fn to_lower_case(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(input.to_lowercase())
}

fn to_upper_case(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(input.to_uppercase())
}

fn remove_spaces(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(input.replace(" ", ""))
}

fn make_slugify(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(slugify(input))
}

fn reverse_string(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(input.chars().rev().collect())
}

fn to_binary(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;
    Ok(input
        .chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<String>>()
        .join(" "))
}

fn csv(path: &str) -> Result<String, Box<dyn Error>> {
    validate_input(path)?;

    let mut reader = csv::Reader::from_path(path)?;

    let mut records = Vec::new();
    let headers = reader.headers()?;
    records.push(headers.iter().collect::<Vec<&str>>().join(" "));

    for result in reader.records() {
        let record = result?;
        records.push(record.iter().collect::<Vec<&str>>().join(" "));
    }

    Ok(records.join("\n"))
}

fn validate_input(input: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Err(Box::from("String is empty"));
    }
    if input.len() > 100 {
        return Err(Box::from("String is too long (max 100 characters)"));
    }
    Ok(())
}
