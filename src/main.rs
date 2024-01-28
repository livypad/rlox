mod run;
mod scanner;

use std::env;

use run::{run_prompt,run_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        // println!("hello");
        run_file(&args[1]);
    } else {
        run_prompt();
        // println!("Usage: rlox [script]");
    }
    // println!("Hello, world!");
}
