pub mod builder;
pub mod private_app;
pub mod publisher;


use std::io;
use std::io::Write;

fn main() {
    println!("1) Create private app");
    println!("2) Update private app");
    print!("Choose an option: ");

    io::stdout().flush().unwrap();

    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Failed to read line");

    let choice: u8 = match choice_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid option");
            return;
        }
    };

    match choice {
        1 => {
            let (tenant, token, path) = ask_infos();
            private_app::create_privateapp(&tenant, &token, &path)
                .expect("Error creating private app");
        }
        2 => {
            let (tenant, token, path) = ask_infos();
            private_app::update_privateapp(&tenant, &token, &path)
                .expect("Error update private app");
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn ask_infos() -> (String, String, String) {
    let mut token_input = String::new();
    let mut tenant_input = String::new();
    let mut path_input = String::new();

    println!("Please enter spreadsheet path:");
    io::stdin()
        .read_line(&mut path_input)
        .expect("Failed to read line");

    println!("Type your token: ");
    io::stdin()
        .read_line(&mut token_input)
        .expect("Failed to read line");

    println!("Type your Tenant URL: Example: https://company.goskope.com you type company ");
    io::stdin()
        .read_line(&mut tenant_input)
        .expect("Failed to read line");

    let tenant = tenant_input.trim().to_string();
    let token = token_input.trim().to_string();
    let path = path_input.trim().to_string();

    (tenant, token, path)
}
