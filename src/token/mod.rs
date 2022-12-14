#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

impl TokenType {
    pub fn string(&self) -> String {
        let s = match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::EOF => "EOF",

            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",

            TokenType::Assign => "=",
            TokenType::Plus => "+",

            TokenType::Comma => ",",
            TokenType::Semicolon => ";",

            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",

            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
        };

        String::from(s)
    }

    pub fn from(c: char) -> Self {
        match c {
            '=' => TokenType::Assign,
            '+' => TokenType::Plus,

            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,

            '(' => TokenType::Lparen,
            ')' => TokenType::Rparen,
            '{' => TokenType::Lbrace,
            '}' => TokenType::Rbrace,

            //'fn' => TokenType::Function,
            //'let' => TokenType::Let,

            //"INT" => TokenType::Int(s.parse().unwrap()),
            //"IDENT" => TokenType::Ident(String::from(s)),
            _ => TokenType::Illegal,
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

#[cfg(test)]
mod token_tests {

    #[test]
    fn hello() {}
}
