use std::process::exit;

use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    StringLiteral(String),
    NumberLiteral(i64),
    BooleanLiteral(bool),
    BuiltinFn(String),

    Keyword(String),
    Identifier(String),

    // Brackets
    ParenLeft,
    ParenRight,
    CurlyLeft,
    CurlyRight,
    SquareLeft,
    SquareRight,

    // Operators
    OpAdd,
    OpSub,
    OpDiv,
    OpMul,
    OpAssign,
    OpEq,
    OpUneq,
    OpLess,
    OpMore,
    OpLessEq,
    OpMoreEq,
    OpNot,
    OpSemicolon,
    OpColon,
    OpMod,
    BitwiseXor,
    BitwiseAnd,
    BitwiseOr,
    OpPeriod,
    OpNewline,
    OpComma,

    End,
}

const KEYWORDS: [&str; 10] = [
    "let", "mut", "pub", "fn", "struct", "enum", "from", "import", "or", "and",
];

pub struct Scanner {
    source: String,
    current: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String, current: usize) -> Scanner {
        Scanner {
            source,
            current,
            tokens: vec![],
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        return self.source.chars().nth(self.current);
    }

    #[inline(always)]
    fn get_current(&mut self) -> Option<char> {
        return self.source.chars().nth(self.current);
    }

    fn identifier(&mut self) -> Token {
        let mut string: String = String::new();

        while let Some(c) = self.get_current() {
            match c {
                'A'..='Z' | 'a'..='z' | '_' => string.push(c),
                _ => break,
            }

            self.advance();
        }

        self.current -= 1;
        return if KEYWORDS.contains(&string.as_str()) {
            Token::Keyword(string)
        } else {
            Token::Identifier(string)
        };
    }

    fn number(&mut self) -> Token {
        let mut num: i64 = 0;

        while let Some(c) = self.get_current() {
            match c {
                '0'..='9' => num = num * 10 + c.to_digit(10).unwrap() as i64,
                _ => break,
            }

            self.advance();
        }

        self.current -= 1;
        return Token::NumberLiteral(num);
    }

    fn string(&mut self) -> Token {
        let mut string: String = String::new();
        let mut escaping: bool = false;

        while let Some(c) = self.advance() {
            match c {
                '\\' => escaping = true,
                '"' => {
                    if !escaping {
                        break;
                    } else {
                        string.push('"')
                    }
                }
                _ => string.push(c),
            }
        }

        return Token::StringLiteral(string);
    }

    pub fn scan(&mut self) {
        while let Some(c) = self.get_current() {
            if c.is_whitespace() {  
                if c == '\n' {
                    self.tokens.push(Token::OpNewline);
                }

                self.advance();
                continue;
            }

            let token = match c {
                'a'..='z' | 'A'..='Z' => self.identifier(),
                '0'..='9' => self.number(),
                '.' => Token::OpPeriod,
                '+' => Token::OpAdd,
                '-' => Token::OpSub,
                '*' => Token::OpMul,
                '/' => Token::OpDiv,
                '!' => Token::OpNot,
                '%' => Token::OpMod,
                ':' => Token::OpColon,
                ';' => Token::OpSemicolon,
                '=' => Token::OpAssign,
                '{' => Token::CurlyLeft,
                '}' => Token::CurlyRight,
                '(' => Token::ParenLeft,
                ')' => Token::ParenRight,
                '[' => Token::SquareLeft,
                ']' => Token::SquareRight,
                '>' => Token::OpMore,
                '<' => Token::OpLess,
                '^' => Token::BitwiseXor,
                '|' => Token::BitwiseOr,
                '&' => Token::BitwiseAnd,
                ',' => Token::OpComma,
                '"' => self.string(),
                a => unimplemented!("{:?}", a),
            };

            self.tokens.push(token.clone());
            self.advance();
        }
    }
}
