mod chat;
mod shorterner;
mod tmanager;

use chat::server::Server;
use shorterner::do_shortener;
use std::io::stdin;
use tmanager::do_task_manager;

fn main() {
    return debug_main();

    println!("What program do you want to run?");
    println!("1. URL Shortener");
    println!("2. Task Manager");
    println!("3. Exit");

    let program_option: u8 = read_input()
        .parse()
        .expect("Should be an integer between 1 and 3");

    println!();

    match program_option {
        1 => do_shortener(),
        2 => do_task_manager(),
        _ => return,
    }
}

fn debug_main() {
    let mut server = Server::default();

    server.start();
}

fn read_input() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read input");

    return input.trim().to_string();
}
