#![allow(dead_code, unused)]

use crate::lexer::Lexer;
use crate::models::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut tokens: Vec<Token> = vec![];
    loop {
        match lexer.current() {
            None => break,
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                lexer.advance();
            }
            Some(b'[') => {
                lexer.advance();
                tokens.push(Token::LeftBracket);
            }
            Some(b']') => {
                lexer.advance();
                tokens.push(Token::RightBracket);
            }
            Some(b'{') => {
                lexer.advance();
                tokens.push(Token::LeftBrace);
            }
            Some(b'}') => {
                lexer.advance();
                tokens.push(Token::RightBrace);
            }
            Some(b':') => {
                lexer.advance();
                tokens.push(Token::Colon);
            }
            Some(b',') => {
                lexer.advance();
                tokens.push(Token::Comma);
            }
            Some(b'"') => {
                let string = lexer.read_string();
                tokens.push(Token::StringToken(string));
            }
            Some(b't') => {
                lexer.read_keyword("true");
                tokens.push(Token::True)
            }
            Some(b'f') => {
                lexer.read_keyword("false");
                tokens.push(Token::False)
            }
            Some(b'n') => {
                lexer.read_keyword("null");
                tokens.push(Token::Null);
            }
            Some(b'-') | Some(b'0'..=b'9') => {
                let num = lexer.read_number();
                tokens.push(Token::NumberToken(num));
            }
            Some(c) => {
                panic!(
                    "Unexpected character while reading the input: {}",
                    c as char
                )
            }
        }
    }
    tokens
}
