use std::io;
use std::io::prelude::*;
use colored::*;
use std::fmt;
use std::mem::size_of;

#[macro_use] extern crate scan_fmt;


const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

const ROW_SIZE: usize = size_of::<Row>();
const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize =  100;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;


struct Table {
    num_rows: usize,
    pages: Vec<Page>
}

impl Table {
    fn insert_row(&mut self, row_to_insert: Row){
        let page_num = self.num_rows / ROWS_PER_PAGE;

        self.pages[page_num].rows.push(row_to_insert);
        self.pages[page_num].num_rows += 1;
    }
}

struct Page {
    num_rows: usize,
    rows: Vec<Row> 
  }

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

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", self.id, self.username, self.email)
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

enum ExecuteResult {
    EXECUTE_SUCCESS,
    EXECUTE_FAILURE,
    EXECUTE_TABLE_FULL
}

fn new_table() -> Table {
    Table {
        num_rows: 0,
        pages: vec![Page{
            num_rows: 0,
            rows: Vec::with_capacity(ROWS_PER_PAGE) 
        }]
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

    if(&command[..6] == "select"){
        statement.statementType = StatementTypes::StatementSelect;
        return PrepareResult::PrepareSuccess; 
    }

    return PrepareResult::PrepareUnrecognized;
}



// Table read / writes 
fn execute_insert(statement: Statement, table: &mut Table) -> ExecuteResult {
    if(table.num_rows >= TABLE_MAX_ROWS){
        return ExecuteResult::EXECUTE_TABLE_FULL;
    }
    table.insert_row(statement.row_to_insert);
    table.num_rows += 1;

    return ExecuteResult::EXECUTE_SUCCESS
}

fn execute_select(statement: Statement, table: &Table) -> ExecuteResult{
    for page in table.pages.iter() {
        for row in page.rows.iter() {
            print!("{}", row)
        }
    }

    return ExecuteResult::EXECUTE_SUCCESS;
}

fn execute_statement(statement: Statement, table: &mut Table) -> ExecuteResult {
 match statement.statementType {
    StatementTypes::StatementInsert => {
        return execute_insert(statement, table)
    },
    StatementTypes::StatementSelect => {
       return execute_select(statement, table)
    },
    StatementTypes::Unknown => {
        println!("{}! This command is unknown", "Error".red());
        return ExecuteResult::EXECUTE_FAILURE;
    } 
  }
}


fn main() {
    let mut table: Table = new_table();
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

        let mut statement = Statement::default();

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

        match execute_statement(statement, &mut table) {
            ExecuteResult::EXECUTE_SUCCESS => {
                println!("{}", "Executed".green());
            },
            ExecuteResult::EXECUTE_FAILURE => {
                println!("{}", "Execution failure".red());
            },
            ExecuteResult::EXECUTE_TABLE_FULL => {
                println!("{}", "Error: Table full".red());
            }
        }
    }
}
