mod parser;

use parser::{Lexer, Config};

use regex::Regex;

fn main() {
    // let lexer = Lexer::new(String::from("test"), Config::new());
    // println!("{:?}", lexer);
    // println!("src is {}", lexer.lex());

    let re = Regex::new(r"(?m)^ +$").unwrap();
    println!("{}", re.replace_all("123\n   \n123\n       \n123", "12345"));
}
