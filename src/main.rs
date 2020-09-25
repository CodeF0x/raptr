use std::env;

mod commands;
mod io;
mod web;

fn main() {
    io::setup();
    let args: Vec<String> = env::args().skip(1).collect();
    commands::handle_arguments(args);
}
