use colored::Colorize;
use std::process::exit;

use crate::{error, scanner::Token};

pub enum Type {
    String,
    Number,
    Float,
    Boolean,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Node {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),

    VariableAssignment {
        name: String,
        value: Box<Node>,
        mutable: bool,
    },
    VariableDestructureAssignment {
        properties: Vec<(String, bool /* mutable */)>,
        value: Box<Node>, // can only be an identifier
        mutable: bool,
    },
    Binary {
        left: Box<Node>,
        right: Box<Node>,
        operation: Token,
    },
    Unary {
        left: Box<Node>,
        operation: char,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Node>,
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize, // current token
    source: String,
    current_line: usize,

    pub ast: Vec<Node>,
    pub warnings: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: String) -> Parser {
        Parser {
            tokens,
            current: 0,
            source,
            ast: vec![],
            current_line: 0,
            warnings: 0,
        }
    }

    fn advance(&mut self) -> Token {
        self.current += 1;

        if self.get_current() == Token::OpNewline {
            self.current_line += 1;
        }

        if self.current < self.tokens.len() {
            return self.get_current();
        } else {
            return Token::End;
        }
    }

    fn expect(&mut self, token: Token, error: &str) {
        if self.advance() != token {
            self.error(error);
        }
    }

    fn error(&mut self, error: &str) -> ! {
        error::print(
            error,
            &self.source.split('\n').collect::<Vec<&str>>(),
            self.current_line,
            0,
            error::ErrorType::Fatal,
        );

        println!("{}", "Could not compile due to error above.".red());
        exit(0);
    }

    fn warn(&mut self, warning: &str) {
        error::print(
            warning,
            &self.source.split('\n').collect::<Vec<&str>>(),
            self.current_line,
            0,
            error::ErrorType::Warning,
        );
        println!("");

        self.warnings += 1;
    }

    #[inline(always)]
    fn is_at_end(&mut self) -> bool {
        self.get_current() == Token::End
    }

    #[inline(always)]
    fn get_current(&mut self) -> Token {
        if self.current < self.tokens.len() {
            self.tokens[self.current].clone()
        } else {
            Token::End
        }
    }

    fn value(&mut self) -> Node {
        let current = self.get_current();
        self.advance();

        match current {
            Token::BooleanLiteral(boolean) => Node::Boolean(boolean),
            Token::NumberLiteral(number) => Node::Number(number),
            Token::StringLiteral(ref string) => Node::String(string.to_string()),
            Token::Identifier(ref string) => {
                let identifier = string.to_string();
                
                if self.get_current() == Token::ParenLeft {
                    let mut arguments: Vec<Node> = vec![];
                    self.advance();

                    loop {
                        if self.get_current() == Token::ParenRight {
                            self.advance();
                            break;
                        } else if self.get_current() == Token::OpComma {
                            self.advance();
                        } else {
                            arguments.push(self.expression());
                            
                            // println!("{:?}", self.get_current());
                            // self.error("SyntaxError: Expected `)` or `,`.")
                        }
                    }

                    Node::FunctionCall {
                        name: identifier,
                        arguments
                    }
                } else {
                    Node::Identifier(identifier)
                }
            },
            Token::OpNot => match self.get_current() {
                Token::BooleanLiteral(_)
                | Token::StringLiteral(_)
                | Token::NumberLiteral(_)
                | Token::Identifier(_)
                | Token::OpNot
                | Token::OpSub => Node::Unary {
                    left: Box::new(self.value()),
                    operation: '!',
                },
                a => self.error(&format!(
                    "SyntaxError: Unexpected token [2] `{:?}`. Expected value",
                    a
                )),
            },
            Token::OpSub => match self.get_current() {
                Token::BooleanLiteral(_)
                | Token::StringLiteral(_)
                | Token::NumberLiteral(_)
                | Token::Identifier(_)
                | Token::OpNot
                | Token::OpSub => Node::Unary {
                    left: Box::new(self.value()),
                    operation: '-',
                },
                a => self.error(&format!(
                    "SyntaxError: Unexpected token [3] `{:?}`. Expected value",
                    a
                )),
            },
            Token::ParenLeft => {
                let expression = self.expression();

                if self.get_current() != Token::ParenRight {
                    self.error("SyntaxError: Expected ')' after expression.");
                }

                self.advance();
                expression
            }
            a => self.error(&format!(
                "SyntaxError: Unexpected token [4] `{:?}`. Expected value.",
                a
            )),
        }
    }

    fn from_builder(&mut self, builder: &str) -> Node {
        match builder {
            "unary" => self.value(),
            "additive" => self.additive_expression(),
            _ => panic!("Unknown builder '{}'", builder),
        }
    }

    /* Helper function for parsing binary expression.
       `builder` -> the function you want to use to parse the left and right sides
       `operators` -> the operators you recognize on this precedence level
    */
    fn binary_expression(&mut self, builder: &str, operators: Vec<Token>) -> Node {
        let mut left = self.from_builder(builder);

        while operators.contains(&self.get_current()) {
            let operator = self.get_current();
            self.advance();

            let right = self.from_builder(builder);

            left = Node::Binary {
                left: Box::new(left),
                right: Box::new(right),
                operation: operator,
            };
        }

        left
    }

    fn additive_expression(&mut self) -> Node {
        self.binary_expression("unary", vec![Token::OpAdd, Token::OpSub])
    }

    fn multiplicative_expression(&mut self) -> Node {
        self.binary_expression("additive", vec![Token::OpMod, Token::OpMul, Token::OpDiv])
    }

    #[inline(always)]
    fn expression(&mut self) -> Node {
        self.multiplicative_expression()
    }

    fn variable_init(&mut self) -> Node {
        self.advance();
        let mutable = self.tokens[self.current] == Token::Keyword("mut".to_string());
        if mutable {
            self.advance();
        }

        if let Token::Identifier(name) = self.get_current() {
            self.expect(Token::OpAssign, "Expected assignment operator");
            self.advance();
            let value = self.expression();

            return Node::VariableAssignment {
                name,
                value: Box::new(value),
                mutable,
            };
        } else if self.get_current() == Token::CurlyLeft {
            let mut properties: Vec<(String, bool)> = vec![];

            loop {
                match self.advance() {
                    Token::Identifier(name) => {
                        properties.push((name, mutable));
                    }
                    Token::Keyword(kw) => match kw.as_str() {
                        "mut" => match self.advance() {
                            Token::Identifier(name) => {
                                properties.push((name.clone(), true));

                                if mutable {
                                    self.warn(&format!("Warning: All destructured properties are mutable. `mut` before `{}` is unnecessary.", name))
                                }
                            }
                            _ => self.error("Expected identifier after `mut`."),
                        },

                        _ => self.error("Expected identifier or `mut`."),
                    },
                    _ => self.error("Expeced identifier or `mut`."),
                }

                if self.advance() == Token::CurlyRight {
                    break;
                }
            }

            self.expect(Token::OpAssign, "Expected assignment operator");
            self.advance();

            if let Token::Identifier(name) = self.advance() {
                return Node::VariableDestructureAssignment {
                    properties,
                    value: Box::new(Node::Identifier(name)),
                    mutable,
                };
            } else {
                self.error("SyntaxError: Destructured variable right hand side must be a single identifier")
            }
        } else {
            self.error("SyntaxError: Expected identifier or `{` after `let`");
        }
    }

    fn statement(&mut self) -> Node {
        return match self.get_current() {
            Token::Keyword(ref kw) => match kw.as_str() {
                "let" => self.variable_init(),
                kw => unimplemented!("{:?}", kw),
            },
            Token::NumberLiteral(_)
            | Token::StringLiteral(_)
            | Token::BooleanLiteral(_)
            | Token::OpNot
            | Token::OpSub
            | Token::ParenLeft
            | Token::BuiltinFn(_) => self.expression(),
            Token::OpNewline => {
                self.advance();
                self.statement()
            }
            a => self.error(&format!("SyntaxError: Unexpected token [1] `{:?}`.", a)),
        };
    }

    pub fn parse(&mut self) {
        
        while !self.is_at_end() {
            let node: Node = self.statement();
            self.ast.push(node);
        }
    }
}
