use std::{fs, env};

#[derive(Debug)]
struct Lexer {
    content: String,

    index: i128,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        return Self {
            content,
            index: 0,
        }
    }
    pub fn lex(&mut self) {
        
    }
}

fn main() {
    let not_sure_file = env::args().nth(1);

    let file = if not_sure_file.is_some() {
        not_sure_file.unwrap()
    } else {
        panic!("File Not Found :))");
    };

    let not_sure_content = fs::read_to_string(file);

    let content = if not_sure_content.is_ok() {
        not_sure_content.unwrap()
    } else {
        panic!("Could not read file :))");
    };

    let lexer = Lexer::new(content);
    // lexer.lex();
}