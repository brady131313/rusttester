use std::process;

fn main() {
    if let Err(err) = rusttester::run() {
        println!("Application error: {}", err);
        process::exit(1);
    }
}
