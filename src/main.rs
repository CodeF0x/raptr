use std::env;

mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = commands::build_command(args);
    match commands::handle_command(command) {
        Ok(()) => (),
        Err(err) => eprintln!("{}", err),
    };
}
