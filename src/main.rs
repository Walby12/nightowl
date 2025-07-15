#[derive(Debug)]
enum Tokens {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equals,
    Let,
    Num(i64),
    Ident(String),
}

fn tokenize(src: String) -> Vec<Tokens> {
    let mut toks: Vec<Tokens> = Vec::new();
    let code: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < code.len() {
        let c = code[i];

        match c {
            ' ' | '\n' | '\t' => i += 1,
            '+' => {
                toks.push(Tokens::Plus);
                i += 1;
            }
            '-' => {
                toks.push(Tokens::Minus);
                i += 1;
            }
            '*' => {
                toks.push(Tokens::Multiply);
                i += 1;
            }
            '/' => {
                toks.push(Tokens::Divide);
                i += 1;
            }
            '=' => {
                toks.push(Tokens::Equals);
                i += 1;
            }
            c if c.is_alphabetic() => {
                let mut builder_str = String::new();
                while i < code.len() && code[i].is_alphabetic() {
                    builder_str.push(code[i]);
                    i += 1;
                }
                match builder_str.as_str() {
                    "let" => toks.push(Tokens::Let),
                    _ => toks.push(Tokens::Ident(builder_str)),
                }
            }
            c if c.is_numeric() => {
                let mut builder_str = String::new();
                while i < code.len() && code[i].is_numeric() {
                    builder_str.push(code[i]);
                    i += 1;
                }
                match builder_str.parse::<i64>() {
                    Ok(num) => toks.push(Tokens::Num(num)),
                    Err(e) => println!("Failed to parse: {}", e),
                }
            }
            _ => {
                println!("Unknwon token: {}", c);
                i += 1;
            }
        }
    }

    toks
}

fn main() {
    let toks = tokenize(String::from("let num = 1234 - 1234"));
    println!("{:?}", toks);
}

