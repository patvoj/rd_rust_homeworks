use slug::slugify;
use std::error::Error;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Invalid input.");
        exit(1);
    }

    let op = &args[1];
    let text = &args[2..].join("");

    let res = match op.as_str() {
        "lowercase" => to_lower_case(&text),
        "upercase" => to_upper_case(&text),
        "no-spaces" => remove_spaces(&text),
        "slugify" => make_slugify(&text),
        "reverse" => reverse_string(&text),
        "binary" => to_binary(&text),
        "csv" => csv(&text),
        _ => {
            eprintln!("Invalid operation: {}", op);
            exit(1);
        }
    };

    match res {
        Ok(output) => println!("{}", output),
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
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

fn csv(input: &str) -> Result<String, Box<dyn Error>> {
    validate_input(input)?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .trim(csv::Trim::All)
        .from_reader(input.as_bytes());

    let mut records = Vec::new();
    records.push(reader.headers()?.clone());
    records.extend(reader.records().filter_map(Result::ok));

    let output = records
        .iter()
        .map(|record| record.iter().collect::<Vec<&str>>().join(" "))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(output)
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
