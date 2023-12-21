use std::{env};
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 2 {
        let command:&String = &args[0];
        let package:&String = &args[1];

        if command == "install" { install(package).await }
        else if command == "uninstall" { uninstall(package) }
        else { println!("Invalid usage!") }
    }
    else { print!("Invalid usage!") }
}

async fn install(package: &String) {
    let client: Client = Client::new();

    let url: String = format!("https://github.com/ahakahacker/mpm/raw/master/packages/{}/build.zip", package);

    let response = client.get(url).send().await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Installing package: {}", package)
            } else {
                println!("Package {} not found", package);
            }
        }
        Err(err) => {
            println!("Ошибка при выполнении запроса: {}", err);
        }
    }
}

fn uninstall(package: &String) {
    println!("Uninstalling package: {}", package)
}