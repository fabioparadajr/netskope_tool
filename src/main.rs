
// Futura documentação: verificar se as colunas do Excel estão em modo texto, jusatmente para aqueles casos onde se utiliza mais deum valor, exemplo "80,443"
// /home/fabio/RustroverProjects/netskope_tool/target/debug/applications.xlsx
pub mod private_app;
pub mod publisher;

use calamine::Reader;
use std::error::Error;
use std::io;
use std::io::Write;

fn main() {
    println!("1) Get publishers");
    println!("2) Create private app");
    print!("Choose an option: ");

    // Força o print sair antes de ler
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

            publisher::get_publisher(&tenant, &token);
        }

        2 => {
            let (tenant, token, path) = ask_infos();
            private_app::create_privateapp(&tenant, &token, &path).expect("Error creating private app");
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

    println!("Type your Tenant URL: Example: https://claro.goskope.com");
    io::stdin()
        .read_line(&mut tenant_input)
        .expect("Failed to read line");

    let tenant = tenant_input.trim().to_string();
    let token = token_input.trim().to_string();
    let path = path_input.trim().to_string();
    


    (tenant, token, path)
}
