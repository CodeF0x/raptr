#![feature(proc_macro_hygiene, decl_macro)]

use std::env;

mod commands;
mod web;


#[macro_use] extern crate rocket;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = commands::build_command(args);
    match commands::handle_command(command) {
        Ok(()) => (),
        Err(err) => eprintln!("{}", err),
    };
}
