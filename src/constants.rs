use crate::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub enum TokenProto {
    // General keywords
    MethodIdent,
    Path,
    Method,
    Class,
    // Symbols
    At,
    LParen,
    RParen,
    Colon,
    Comma,
    // Other
    Identifier,
    Number,
    String,
    Boolean,
    Char,
    Comment,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    // General keywords
    MethodIdent(String),
    Path,
    Method,
    Class,
    // Symbols
    At,
    LParen,
    RParen,
    Colon,
    Comma,
    // Other
    Identifier(String),
    Number(String),
    String(String),
    Char(String),
    Boolean(String),
    Comment(String),
}

impl TokenProto {
    pub fn to_token(&self, input: String) -> Token {
        match self {
            // General keywords
            TokenProto::MethodIdent => Token::MethodIdent(input),
            TokenProto::Path => Token::Path,
            TokenProto::Method => Token::Method,
            TokenProto::Class => Token::Class,
            // Symbols
            TokenProto::At => Token::At,
            TokenProto::LParen => Token::LParen,
            TokenProto::RParen => Token::RParen,
            TokenProto::Colon => Token::Colon,
            TokenProto::Comma => Token::Comma,
            // Other
            TokenProto::Identifier => Token::Identifier(input),
            TokenProto::Number => Token::Number(input),
            TokenProto::String => Token::String(input),
            TokenProto::Boolean => Token::Boolean(input),
            TokenProto::Char => Token::Char(input),
            TokenProto::Comment => Token::Comment(input),
        }
    }
}

impl Token {
    pub fn proto(&self) -> TokenProto {
        match self {
            // General keywords
            Token::MethodIdent(_) => TokenProto::MethodIdent,
            Token::Path => TokenProto::Path,
            Token::Method => TokenProto::Method,
            Token::Class => TokenProto::Class,
            // Symbols
            Token::At => TokenProto::At,
            Token::LParen => TokenProto::LParen,
            Token::RParen => TokenProto::RParen,
            Token::Colon => TokenProto::Colon,
            Token::Comma => TokenProto::Comma,
            // Other
            Token::Identifier(_) => TokenProto::Identifier,
            Token::Number(_) => TokenProto::Number,
            Token::String(_) => TokenProto::String,
            Token::Boolean(_) => TokenProto::Boolean,
            Token::Char(_) => TokenProto::Char,
            Token::Comment(_) => TokenProto::Comment,
        }
    }
}

pub const LEX_AHOCOR_DICT: Lazy<Vec<(TokenProto, Vec<&'static str>)>> = Lazy::new(|| vec![
    (TokenProto::Path, vec!["path"]),
    (TokenProto::Method, vec!["method"]),
    (TokenProto::Class, vec!["class"]),
    (TokenProto::At, vec!["@"]),
    (TokenProto::LParen, vec!["("]),
    (TokenProto::RParen, vec![")"]),
    (TokenProto::Colon, vec![":"]),
    (TokenProto::Comma, vec![","]),
    (TokenProto::Boolean, vec!["true", "false"]),
]);

pub const LEX_REGEX_DICT: Lazy<Vec<(TokenProto, Regex)>> = Lazy::new(|| vec![
    (TokenProto::Identifier, Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap()),
    (TokenProto::Number, Regex::new(r"\b-?\d+(\.\d+)?\b").unwrap()),
    (TokenProto::Char, Regex::new(r"'([^']|''{1})'").unwrap()),
    (TokenProto::String, Regex::new(r"'([^']|'')*'").unwrap()),
    (TokenProto::Comment, Regex::new(r"\{[^}]*\}").unwrap())
]);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DataType {
    Int,
    String,
    Char,
    Bool,
}

impl FromStr for DataType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "integer" => Ok(DataType::Int),
            "string" => Ok(DataType::String),
            "char" => Ok(DataType::Char),
            "boolean" => Ok(DataType::Bool),
            _ => bail!(format!("'{}' is not a valid datatype", s))
        }
    }
}