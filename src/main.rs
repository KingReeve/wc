use std::env;
use std::fs;
use std::io::{self, Read};

fn count_chars(file: &str) -> u64 {
    file.chars().count() as u64
}

fn count_words(file: &str) -> u64 {
    file.split_whitespace().count() as u64
}

fn count_bytes(file: &str) -> u64 {
    file.len() as u64
}

fn count_lines(file: &str) -> u64 {
    file.chars().filter(|&c| c == '\n').count() as u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wc <file_path> [options]");
        eprintln!("Options: w (words), l (lines), c (bytes), m (chars)");
    }

    let (contents, file_path) = if args.len() < 2 {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => (buffer, None),
            Err(e) => {
                eprintln!("Failed to read from stdin: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        let file_path = &args[1];
        match fs::read_to_string(file_path) {
            Ok(contents) => (contents, Some(file_path)),
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    };

    let file_path_str = file_path.unwrap_or(&"".to_string()).to_string();

    if args.len() <= 2 {
        println!(
            "{} {} {} {}",
            count_lines(&contents),
            count_words(&contents),
            count_bytes(&contents),
            file_path_str
        );
        std::process::exit(0);
    }

    for i in &args[2..] {
        if i == "-c" {
            println!("{} {}",count_bytes(&contents),file_path_str);
        }
        if i == "-l" {
            println!("{} {}",count_lines(&contents),file_path_str);
        }
        if i == "-w" {
            println!("{} {}",count_words(&contents),file_path_str);
        }
        if i == "-m" {
            println!("{} {}",count_chars(&contents),file_path_str);
        }
    }
    std::process::exit(0);
}