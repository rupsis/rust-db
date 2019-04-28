use std::io;
use std::io::prelude::*;
use colored::*;

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognized
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognized
}


fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn process_meta_command(command: &String) -> MetaCommandResult {
     if (command == ".exit"){
        std::process::exit(0);
    } else {
        return MetaCommandResult::MetaCommandUnrecognized;
    }
}

// fn process_prepare_statement(command: &String) -> PrepareResult {
// }

fn main() {
    println!("Starting Rust DB");

    while(true){
        print_prompt();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("error: enable to read input");
        
        user_input.truncate(user_input.len() - 1);

        if(&user_input[0..1] == "."){
            match process_meta_command(&user_input) {
                MetaCommandResult::MetaCommandSuccess => {
                    println!("This is a meta command");
                },
                MetaCommandResult::MetaCommandUnrecognized => {
                    println!("Unrecognized command {}", user_input.red());
                },
            }      
        }
    }
}
