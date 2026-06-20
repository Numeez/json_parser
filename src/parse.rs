#![allow(unused, dead_code)]

use crate::models::{JsonValue, Token};

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }
    fn current(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            panic!("Unexpected end of token streams")
        }
    }
    fn advance(&mut self) {
        self.pos += 1
    }
    fn expect(&mut self, description: &str) -> &Token {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            token
        } else {
            panic!("Unexpected end of token streams: {}", description)
        }
    }

    fn parse_value(&mut self) -> JsonValue {
        match self.current() {
            Token::LeftBrace => self.parse_object(),
            Token::LeftBracket => self.parse_array(),
            Token::True => {
                self.advance();
                JsonValue::Bool(true)
            }
            Token::False => {
                self.advance();
                JsonValue::Bool(false)
            }
            Token::Null => {
                self.advance();
                JsonValue::Null
            }
            Token::NumberToken(n) => {
                let value = *n;
                self.advance();
                JsonValue::Number(value)
            }
            Token::StringToken(str_token) => {
                let value = str_token.clone();
                self.advance();
                JsonValue::Str(value)
            }
            Token::RightBrace => panic!("Unexpected '}}'"),
            Token::RightBracket => panic!("Unexpected ']'"),
            Token::Colon => panic!("Unexpected ':'"),
            Token::Comma => panic!("Unexpected ','"),
        }
    }

    fn parse_object(&mut self) -> JsonValue {
        self.advance();
        let mut result: Vec<(String, JsonValue)> = vec![];
        if let Token::RightBrace = self.current() {
            self.advance();
            return JsonValue::Object(result);
        }
        loop {
            let key = match self.expect("object key") {
                Token::StringToken(str_token) => str_token.clone(),
                _ => {
                    panic!("Expected a key got: {:?}", self.current())
                }
            };
            match self.expect("colon") {
                Token::Colon => {}
                _ => panic!("Expected ':' after object key"),
            }
            let value = self.parse_value();
            result.push((key, value));

            match self.current() {
                Token::Comma => {
                    self.advance();
                }
                Token::RightBrace => {
                    self.advance();
                    break;
                }
                _ => panic!("Invalid json"),
            }
        }
        JsonValue::Object(result)
    }

    fn parse_array(&mut self) -> JsonValue {
        let mut result: Vec<JsonValue> = vec![];
        self.advance();

        if let Token::RightBracket = self.current() {
            self.advance();
            return JsonValue::Array(result);
        }
        loop {
            let element = self.parse_value();
            result.push(element);
            match self.current() {
                Token::Comma => {
                    self.advance();
                }
                Token::RightBracket => {
                    self.advance();
                    break;
                }
                _ => panic!("Invalid json array format"),
            }
        }

        JsonValue::Array(result)
    }
}

pub fn parse(tokens: Vec<Token>) -> JsonValue {
    let mut parser = Parser::new(tokens);
    let value = parser.parse_value();
    value
}
