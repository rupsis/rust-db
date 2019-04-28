use std::io;
use std::io::prelude::*;

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn main() {
    println!("Starting Rust DB");

    while(true){
        print_prompt();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("error: enable to read input");

        println!("{}", user_input);

        user_input.truncate(user_input.len() - 1);
        if (user_input == ".exit"){
            std::process::exit(0);
        } else {
            println!("Unrecognized command {}", user_input)
        }
    }

}
