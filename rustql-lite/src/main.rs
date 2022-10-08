pub mod enums;

use std::io;
use std::io::Write;
use std::process;

use enums::meta_command_result::MetaCommandResult;
use enums::prepare_result::PrepareResult;
use enums::statement_type::StatementType;

struct Statement {
    statement_type: StatementType,
}

fn main() {
    println!("Welcome to rustql-lite command line!");

    loop {
        print_prompt();

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: &str = input.trim();

        if input.starts_with(".") {
            match do_meta_command(input) {
                MetaCommandResult::MetaCommandSuccess => process::exit(0x100),
                MetaCommandResult::MetaCommandUnrecognizedCommand => println!("Invalid meta command"),
            };
        }
    
        let mut statement: Statement = Statement {statement_type: StatementType::StatementBlank};

        let prepared_result: PrepareResult = prepare_statement(&input, &mut statement);

        match prepared_result {
            PrepareResult::PrepareSuccess => (),
            PrepareResult::PrepareUnrecognizedCommand => println!("Unrecognized command"),
        };

        execute_statement(&statement);
        println!("Executed code");
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn do_meta_command(command: &str) -> MetaCommandResult {
    if command == ".exit" {
        MetaCommandResult::MetaCommandSuccess
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn prepare_statement(command: &str, statement: &mut Statement) -> PrepareResult {
    if command.starts_with("insert") {
        statement.statement_type = StatementType::StatementInsert;
        PrepareResult::PrepareSuccess
    } else if command == "select" {
        statement.statement_type = StatementType::StatementSelect;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedCommand
    }
}

fn execute_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::StatementInsert => println!("This is where we insert"),
        StatementType::StatementSelect => println!("This is where we select"),
        StatementType::StatementBlank => (),
    };
}