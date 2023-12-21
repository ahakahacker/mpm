use std::{env};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 2 {
        let command:&String = &args[0];
        let package:&String = &args[1];

        if command == "install" {
            println!("Installing package: {}", package)
        }
        else if command == "uninstall" {
            println!("Uninstalling package: {}", package)
        }
    }
    else { print!("Invalid usage") }
}