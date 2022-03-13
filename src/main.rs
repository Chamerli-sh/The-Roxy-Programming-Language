use std::{fs, env};


struct Lexer {
    content: String,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        return Self {content}
    }
}

fn main() {
    let file = env::args().nth(1).unwrap();

    let content = fs::read_to_string(file).unwrap();
    println!("{}", content);
}