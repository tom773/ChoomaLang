#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]

mod utils;
mod test;
mod libchooma;
mod scanner;
use logos::Logos;

use std::{
    env,
    io::{BufRead, Write},
    process::exit,
};

// Chooma Lang v0.1

fn run_file(path: &str) {
    let source = std::fs::read_to_string(path)
        .expect("Failed to read file");
    let _ = run(&source);
}

fn run(contents: &str) -> Result<(), String>{
     
    let lexer = scanner::Token::lexer(contents);
     
    utils::eval::testutils(lexer);

    Ok(())
}

fn run_prompt() -> Result<(), String>{
    loop {
        print!("> ");
        match std::io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Failed to flush stdout".to_string()),
        }
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Failed to read line".to_string()),
        }
        match run(&buffer){
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        
        println!("Usage: Chooma <filename>");
        exit(64);

    } else if args.len() == 2 {

        run_file(&args[1]);

    } else {
        
        let _ = run_prompt();

    } 
}


