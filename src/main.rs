use std::{
    env,
    error::Error,
    io::{self, Write},
    path::Path,
    process::exit
};
use colorize::AnsiColor;

fn main() {
    loop {
        let current_dir = env::current_dir().unwrap();
        let head_display = println!("{} 👉 ",current_dir);
        println!(head_display);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                let mut parts = input.split_whitespace();
                let command = parts.next().unwrap();
                let args : Vec<&str> = parts.collect();
                command_matcher(command, args);
            }
            Err(error) => {
                eprintln!("error: {}", error);
                break;
            }
        }

    }
}

fn command_matcher(command: &str, args: Vec<&str>) {
    match command {
        "cd" => {
            cmd_cd(args);
        }
        "exit" => {
            cmd_exit();
        }
        command => {
            cmd_other(command, args);
        }
    }
}

fn cmd_cd(args: Vec<&str>) {
    let new_dir = args.first().unwrap_or(&"/");
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(root){
        eprintln!("cd: {}", e);
    }
    return;
}

fn cmd_exit() {
    println!("🚪🏃‍");
    exit(0);
}

fn cmd_other(command: &str, args: Vec<&str>) {
    println!("You command : {}", command);
    println!("Args : {:?}", args);
}
