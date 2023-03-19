use colored::Colorize;
use std::{fs::read_to_string, time::Instant};

mod error;
mod parser;
mod scanner;

use parser::Parser;
use scanner::Scanner;

fn main() {
    let start = Instant::now();

    let contents: String = read_to_string("main.lc").unwrap();
    let mut scanner = Scanner::new(contents.clone(), 0);
    scanner.scan();

    let mut parser: Parser = Parser::new(scanner.tokens, contents);
    parser.parse();

    println!("{:#?}", parser.ast);
    println!(
        "{} in {:?} with {}",
        "Compiled".green().bold(),
        start.elapsed(),
        format!(
            "{} {}",
            parser.warnings,
            if parser.warnings == 1 {
                "warning"
            } else {
                "warnings"
            }
        )
        .yellow()
    )

    // error::print(
    //    "Invalid return type. Expected string.",
    //    &vec!["pub fn foo() -> string {", "    return 89;", "}"],
    //    1, 11
    // );
}
