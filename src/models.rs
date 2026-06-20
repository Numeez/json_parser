#![allow(unused, dead_code)]

#[derive(PartialEq, Debug)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    StringToken(String),
    NumberToken(f64),
    True,
    False,
    Null,
}

pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub fn display(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => String::from("null"),
        JsonValue::Bool(true) => String::from("true"),
        JsonValue::Bool(false) => String::from("false"),
        JsonValue::Number(n) => format!("{}", n),
        JsonValue::Str(s) => format!("\"{}\"", s),

        JsonValue::Array(elements) => {
            let mut result = String::from("[");
            let mut first = true;
            for element in elements {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&display(element));
                first = false;
            }
            result.push(']');
            result
        }

        JsonValue::Object(pairs) => {
            let mut result = String::from("{");
            let mut first = true;
            for (key, value) in pairs {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&format!("\"{}\": {}", key, display(value)));
                first = false;
            }
            result.push('}');
            result
        }
    }
}
