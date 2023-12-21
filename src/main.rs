use std::{env};
use std::iter::Cycle;
use reqwest::Client;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 2 {
        let command:&String = &args[0];
        let package:&String = &args[1];

        if command == "install" {
            let client: Client = Client::new();

            let url: String = ""

            println!("Installing package: {}", package)
        }
        else if command == "uninstall" {
            println!("Uninstalling package: {}", package)
        }
        else { println!("Invalid usage!") }
    }
    else { print!("Invalid usage!") }
}