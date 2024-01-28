use std::io;
use std::io::Write;


pub(crate) fn run_prompt() {

    // print!("> ");
    loop{
        print!("> ");
        io::stdout().flush().expect("TODO: panic message");
        let mut input = String::new();
        match io::stdin().read_line(&mut input){
            Ok(n) => {
                println!("{} bytes read", n);
                // println!("{}", input);
                if(input == "exit\n"){
                    break;
                }
            }
            Err(e) => break,
        }
        // run(&guess);
    }
}