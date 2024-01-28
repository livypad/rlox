use std::io::{Read, Write};
use std::{fs, io};

use crate::scanner;

pub(crate) fn run_prompt() {
    // print!("> ");
    loop {
        print!("> ");
        io::stdout().flush().expect("TODO: panic message");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                // println!("{}", input);                
                if input == "exit\n" {
                    break;
                }
                run(input.clone());

            }
            Err(_e) => break,
        }
        // run(&guess);
    }
}

pub(crate) fn run_file(filename: &str) {
    // println!("{}", filename);
    if let Ok(mut file) = fs::File::open(filename) {
        let mut buffer = String::new();
        let _ = file.read_to_string(&mut buffer);
        // println!("{}",buffer);
        run(buffer);
    }
}

fn run(line: String) {
    let scanner = scanner::Scanner::new(line);
    scanner.print_tokens();
}
