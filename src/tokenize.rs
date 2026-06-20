#![allow(dead_code,unused)]

use crate::lexer::Lexer;
use crate::models::Token;

fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    let mut token: Vec<Token> = vec![];
    loop {
        match lexer.current() {
            None => break,
            Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                lexer.advance();
            }
            Some(b'[') => {
                lexer.advance();
                token.push(Token::LeftBracket);
            }
            Some(b']') => {
                lexer.advance();
                token.push(Token::RightBracket);
            }
            Some(b'{') => {
                lexer.advance();
                token.push(Token::LeftBrace);
            }
            Some(b'}') => {
                lexer.advance();
                token.push(Token::RightBrace);
            }
            Some(b':') => {
                lexer.advance();
                token.push(Token::Colon);
            }
            Some(b',') => {
                lexer.advance();
                token.push(Token::Comma);
            }
            Some(b'"')=>{
                let string = lexer.read_string();
                token.push(Token::StringToken(string));
            }
            Some(b't')=>{
                lexer.read_keyword("true");
                token.push(Token::True)
            }
            Some(b'f')=>{
                lexer.read_keyword("false");
                token.push(Token::False)
            }
            Some(b'-') | Some(b'0'..=b'9')=>{
                let num = lexer.read_number();
                token.push(Token::NumberToken(num));
            }
            Some(c) => {
                panic!("Unexpected character while reading the input: {}",c as char)
            }
        }
    }

    todo!()
}
