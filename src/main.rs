use std::{fs, env};


struct Lexer {

}

fn main() {
    let file = env::args().nth(1).unwrap();

    let content = fs::read_to_string(file).unwrap();
    println!("{}", content);
}