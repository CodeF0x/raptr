
use std::env;

mod commands;
mod web;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = commands::build_command(args);
    match commands::handle_command(command) {
        Ok(()) => (),
        Err(err) => eprintln!("{}", err),
    };
}
