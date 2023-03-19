use std::fs::read_to_string;

mod error;
mod parser;
mod scanner;

use parser::Parser;
use scanner::Scanner;

fn main() {
    let contents: String = read_to_string("main.lc").unwrap();
    let mut scanner = Scanner::new(contents.clone(), 0);
    scanner.scan();

    println!("{:?}", scanner.tokens.clone());

    let mut parser: Parser = Parser::new(scanner.tokens, contents);
    parser.parse();

    println!("{:?}", parser.ast);

    // error::print(
    //    "Invalid return type. Expected string.",
    //    &vec!["pub fn foo() -> string {", "    return 89;", "}"],
    //    1, 11
    // );
}
