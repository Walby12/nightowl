use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Tokens {
    Ls,
    Cat(String),
}

fn tokenize(src: &str) -> Vec<Tokens> {
    let mut toks: Vec<Tokens> = Vec::new();
    let code: Vec<&str> = src.split_whitespace().collect();

    let mut i = 0;

    while i < code.len() {
        let c = code[i];
        
        match c {
            "ls" => {
                toks.push(Tokens::Ls);
                i += 1;
            }
            "cat" => {
                toks.push(Tokens::Cat(String::from(code[i+1])));
                i += 2;
            }
            _ => {
                println!("ERROR: Unrecogized option: {}, in {}", c, src);
                assert!(false);
            }
        }
    }
    toks
}

fn parse(toks: Vec<Tokens>) {
    let mut i = 0;

    while i < toks.len() {
        let t = toks[i].clone();

        match t {
            Tokens::Ls => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => println!("ERROR: Could not get the current_dir: {}", e),
                }
                i += 1;
            }
            _ => {
                todo!();
                i += 1;
            }
        }
    }
}

fn main() {
    let toks = tokenize("ls");
    println!("{:?}", toks);
    parse(toks);
}
