use std::io;
use std::io::Write;

fn main() {
    let mut input_buffer = new_input_buffer();
    loop {
        print_prompt();
        get_line(&mut input_buffer);

            if input_buffer.buffer == ".exit" {
                std::process::exit(0);
            } else {
                println!("Unrecognized command: '{}'", input_buffer.buffer);
            }
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
