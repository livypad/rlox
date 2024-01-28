mod run;
mod token;
mod scanner;


use std::env;

use run::run_prompt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 { println!("Hello, world!"); } else {
        run_prompt();
        // println!("Usage: rlox [script]");
    }
    println!("Hello, world!");
}
