use std::env;
use std::fs;
use std::io::{self, Read};

fn count_chars(file:&str) -> u64{
    let mut count:u64 = 0;
    for _i in file.chars(){
        count+=1;
    }
    count
}

fn count_words(file:&str) -> u64{
    let mut count:u64 = 0;
    for _i in file.split_whitespace(){
        count+=1;
    }
    count
}

fn count_bytes(file:&str) -> u64{
    file.len() as u64
}

fn count_lines(file:&str) -> u64{
    let mut count:u64 = 0;
    for i in file.chars(){
        if i=='\n'{
            count+=1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wc <file_path> [options]");
        eprintln!("Options: w (words), l (lines), c (bytes), m (chars)");
    }
    
    let (contents, file_path) =
        if args.len() < 2 {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            (buffer, None)
        } else {
            let file_path = &args[1];
            let contents = fs::read_to_string(file_path).expect("Failed to read file");
            (contents, Some(file_path))
        };
    
    let file_path_str = match file_path {
        Some(name) => format!("{}", name),
        None => "".to_string(),
    };

    if args.len() <= 2 {
        println!(
            "{} {} {} {}",
            count_lines(&contents),
            count_words(&contents),
            count_bytes(&contents),
            file_path_str
        );
        return;
    }

    for i in &args[2..]{
        if i=="c"{
            println!("{} {}",count_bytes(&contents),file_path_str);
        }
        if i=="l"{
            println!("{} {}",count_lines(&contents),file_path_str);
        }
        if i=="w"{
            println!("{} {}",count_words(&contents),file_path_str);
        }
        if i=="m"{
            println!("{} {}",count_chars(&contents),file_path_str);
        }
    }
}
