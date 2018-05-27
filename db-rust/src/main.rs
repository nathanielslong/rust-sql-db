use std::io;
use std::io::Write;

fn main() {
    loop {
        print_prompt();

        let mut input = get_input();
        input = input.trim().to_string();
        if input == ".exit" {
            std::process::exit(0);
        } else {
            println!("Unrecognized command: '{}'", input);
        }
    }
}

fn print_prompt() {
    print!("db > ");

    // needed to get on same line as print
    std::io::stdout().flush().expect("Stdout flush failed, restart the
                                     program I guess? I don't think this
                                     should ever hit...");
}

fn get_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input.");
        input
}
