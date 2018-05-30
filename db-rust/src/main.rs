use std::io;
use std::io::Write;

fn main() {
    let mut input_buffer = new_input_buffer();
    loop {
        print_prompt();
        get_line(&mut input_buffer);

        if input_buffer.buffer.starts_with(".") {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input_buffer.buffer);
                    continue;
                },
                _ => (),
            }
        }

        let mut statement: Statement = Statement {
            statement_type: StatementType::default()
        };
        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.",
                         input_buffer.buffer);
                continue;
            },
            _ => (),
        }
        execute_statement(&statement);
        println!("Executed.")
    }
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    if input_buffer.buffer == ".exit" {
        std::process::exit(0);
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

enum StatementType {
    StatementInsert,
    StatementSelect,
    StatementNone,
}

impl Default for StatementType {
    fn default() -> StatementType { StatementType::StatementNone }
}

struct Statement {
    statement_type: StatementType,
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer == "insert" {
        statement.statement_type = StatementType::StatementInsert;
        return PrepareResult::PrepareSuccess;
    }
    if input_buffer.buffer == "select" {
        statement.statement_type = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    }
    PrepareResult::PrepareUnrecognizedStatement
}

fn execute_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.");
        }
        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
        _ => (),
    }
}

struct InputBuffer {
    buffer: String,
    buffer_length: usize,
    input_length: isize,
}

fn new_input_buffer() -> InputBuffer {
    let input_buffer = InputBuffer {
        buffer: String::new(),
        buffer_length: 0,
        input_length: 0,
    };

    input_buffer
}

fn print_prompt() {
    print!("db > ");

    // needed to get on same line as print
    std::io::stdout().flush().expect("Stdout flush failed, restart the
                                     program I guess? I don't think this
                                     should ever hit...");
}

fn get_line(input_buffer: &mut InputBuffer) {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    input_buffer.buffer = input.trim().to_string();
}
