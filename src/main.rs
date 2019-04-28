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

enum StatementTypes {
    StatementInsert,
    StatementSelect,
    Unknown
}

struct Statement {
    statementType: StatementTypes,
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

fn process_prepare_statement(command: &String, statement: &mut Statement) -> PrepareResult {
    if(command == "insert"){
        statement.statementType = StatementTypes::StatementInsert;
        return PrepareResult::PrepareSuccess; 
    }

    if(command == "select"){
        statement.statementType = StatementTypes::StatementSelect;
        return PrepareResult::PrepareSuccess; 
    }

    return PrepareResult::PrepareUnrecognized;
}

fn execute_statement(statement: &Statement) {
 match statement.statementType {
    StatementTypes::StatementInsert => {
        println!("This is where we would do an insert");
    },
    StatementTypes::StatementSelect => {
        println!("This is where we would do a select");
    },
    StatementTypes::Unknown => {
        println!("{}! This command is unknown", "Error".red());
    } 
  }
}


fn main() {
    println!("{}", "Starting Rust DB".green());

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

        let mut statement = Statement{ statementType: StatementTypes::Unknown};

        match process_prepare_statement(&user_input, &mut statement) {
            PrepareResult::PrepareSuccess => {
                println!("This is a prepare statement command");
            },
            PrepareResult::PrepareUnrecognized => {
                println!("Unrecognized keyword at start of {}", user_input.red());
            },
        } 

        execute_statement(&statement);
        println!("{}", "Executed".green());
    }
}
