use std::io::{self, Write, Read};

mod commands;

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut res = String::new();
    io::stdin().read_to_string(&mut res).unwrap();
}
