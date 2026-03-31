use std::io::{self, Write};

mod control;
use control::{Command};

fn main() {
    println!("=== M2 Control CLI ===");
    println!("Comandos: start | stop | status | exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let cmd = input.trim();

        if cmd == "exit" {
            println!("Saliendo...");
            break;
        }

        let out = match cmd {
            "start" => control::handle(Command::Start),
            "stop" => control::handle(Command::Stop),
            "status" => control::handle(Command::Status),
            _ => "Comando no reconocido".to_string(),
        };

        println!("{out}");
    }
}
