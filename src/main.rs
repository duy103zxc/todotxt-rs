extern crate todotxt_rs;
use std::env;
// use todotxt_rs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        todotxt_rs::help();
        std::process::exit(1);
    }

    let command = &args[1];
    let suffix = &args[2..].iter().cloned().collect::<Vec<_>>().join(" ");
    
    match command.as_str() {
        "add" => {
            
        },
        "list" => {
            
        },
        "update" => {
            
        },
        "del" => {
            
        }
        "help" => {
            todotxt_rs::help();
        }
        _ => {
            todotxt_rs::help();
        }
    }
}

