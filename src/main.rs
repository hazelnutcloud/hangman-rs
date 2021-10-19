use std::process;

fn main() {
    if let Err(e) = hangman::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


