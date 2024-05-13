use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        eprintln!("Program requires at least 2 arguments: <filepath> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();
    let _content = fs::read_to_string(file_path).unwrap();

    for line in _content.lines() {
        if line == search_name {
            print!("{} is here!\n", line);
            return;
        }
    }

    println!("{} is not here...", search_name);
}
