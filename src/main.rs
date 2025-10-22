use std::env;
use std::fs;


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
        eprintln!("Usage: wc_rs <file_path> [options]");
        eprintln!("Options: w (words), l (lines), c (bytes), m (chars)");
        return;
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Error reading file.");

    if args.len() == 2 {
        println!(
            "{} {} {} {}",
            count_lines(&contents),
            count_words(&contents),
            count_bytes(&contents),
            file_path,
        );
        return;
    }

    for i in &args[2..]{
        if i=="c"{
            println!("{} {file_path}",count_bytes(&contents));
        }
        if i=="l"{
            println!("{} {file_path}",count_lines(&contents));
        }
        if i=="w"{
            println!("{} {file_path}",count_words(&contents));
        }
        if i=="m"{
            println!("{} {file_path}",count_chars(&contents));
        }
    }
}
