#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    Int,
    Void,
    Return,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
}
