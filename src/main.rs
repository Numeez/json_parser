#![allow(unused)]

mod lexer;
mod models;
mod parse;
mod tokenize;

fn main() {
    let tests = vec![
        r#"null"#,
        r#"true"#,
        r#"42"#,
        r#"-3.14"#,
        r#""hello world""#,
        r#"[]"#,
        r#"{}"#,
        r#"[1, 2, 3]"#,
        r#"{"name": "alice", "age": 30, "active": true}"#,
        r#"{"scores": [95, 87, 100], "info": {"city": "delhi", "zip": null}}"#,
        r#"["hello\nworld", "tab\there", "\u0041\u0042\u0043"]"#,
    ];

    for input in &tests {
        let tokens = tokenize::tokenize(input);
        let value = parse::parse(tokens);
        println!("Input:  {}", input);
        println!("Output: {}", models::display(&value));
        println!();
    }
}
