use std::process;

fn main() {
    if let Err(e) = hangmanrs::run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
