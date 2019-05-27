use std::io;
use std::io::prelude::*;
use colored::*;

#[macro_use] extern crate scan_fmt;

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

struct Row {
  id: u32,
  username: String,
  email: String,
}

impl Default for Row {
    fn default () -> Row {
        Row { id: 0, username: String::from("nate"), email: String::from("test@test.com")}
    }
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognized
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognized,
    PrepareSyntaxError
}

enum StatementTypes {
    StatementInsert,
    StatementSelect,
    Unknown
}

struct Statement {
    statementType: StatementTypes,
    row_to_insert: Row,
}

impl Default for Statement {
    fn default () -> Statement {
        Statement{
            statementType: StatementTypes::Unknown,
            row_to_insert: Row::default()
        }
    }
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
    if(&command[..6] == "insert"){
        statement.statementType = StatementTypes::StatementInsert;
        let (a, b, c) = scan_fmt_some!(
            &command, 
            "insert {d} {} {}", u32, String, String
            );
        match (a, b, c) {
            (Some(a), Some(b), Some(c)) => {
                statement.row_to_insert.id = a;
                statement.row_to_insert.username = b;
                statement.row_to_insert.email = c;
            }
            _ => return PrepareResult::PrepareSyntaxError
        }
       
        return PrepareResult::PrepareSuccess; 
    }

    // +    int args_assigned = sscanf(
// +        input_buffer->buffer, "insert %d %s %s", &(statement->row_to_insert.id),
// +        statement->row_to_insert.username, statement->row_to_insert.email);
// +    if (args_assigned < 3) {
// +      return PREPARE_SYNTAX_ERROR;
// +    }

    if(&command[..6] == "select"){
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

        let mut statement = Statement::default();;

        match process_prepare_statement(&user_input, &mut statement) {
            PrepareResult::PrepareSuccess => {
                println!("This is a prepare statement command");
            },
            PrepareResult::PrepareUnrecognized => {
                println!("Unrecognized keyword at start of {}", user_input.red());
            },
            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement.")
            }
        } 

        execute_statement(&statement);
        println!("{}", "Executed".green());
    }
}
