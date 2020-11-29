#[macro_use]
extern crate sailfish_macros;

use std::env;

mod commands;
mod io;
mod config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    commands::handle_arguments(args);
}
