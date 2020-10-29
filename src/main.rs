use std::env;

mod commands;
mod io;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    commands::handle_arguments(args);
}
