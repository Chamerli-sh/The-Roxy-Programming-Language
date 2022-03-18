#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Assign,
    Var,
    String  
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind, 
    literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {
            kind,
            literal
        }
    }
}
