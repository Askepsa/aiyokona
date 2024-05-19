#![allow(unused)]
use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Num(i64),
    LParen,
    RParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    PrintMethod,
    Eof,
    Illegal,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: usize,
    next_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
            next_pos: 1,
            ch: ' ',
        }
    }

    fn read_alphabet(&mut self) {
        while let Some(ch) = self.input.chars().nth(self.next_pos) {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            self.next_pos += 1;
        }
    }

    fn read_numeral(&mut self) {
        while let Some(ch) = self.input.chars().nth(self.next_pos) {
            if !ch.is_ascii_digit() {
                break;
            }
            self.next_pos += 1;
        }
    }

    fn eat_whitespace(&mut self) {
        while let Some(ch) = self.input.chars().nth(self.pos) {
            if ch.is_whitespace() {
                self.pos = self.next_pos;
                self.next_pos += 1;
            } else {
                break;
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos >= self.input.len() {
            return None;
        } else {
            self.eat_whitespace();
            self.ch = self.input.chars().nth(self.pos)?;
        }

        let token = {
            match self.ch {
                '(' => Token::LParen,
                ')' => Token::RParen,
                '+' => Token::Plus,
                '-' => {
                    let mut token = Token::Minus;

                    let ch = self.input.chars().nth(self.next_pos)?;
                    if ch.is_ascii_digit() {
                        self.pos += 1;
                        self.read_numeral();

                        let num_str = &self.input.as_str()[self.pos..(self.next_pos - 1)];
                        let num = num_str
                            .parse::<i64>()
                            .expect("parsing of num string failed");
                        token = Token::Num(num * -1);
                    }

                    token
                }
                '*' => Token::Multiply,
                '/' => Token::Divide,
                '\0' => Token::Eof,
                _ => {
                    if self.ch.is_ascii_alphabetic() {
                        self.read_alphabet();

                        let ident = &self.input.as_str()[self.pos..(self.next_pos)];
                        match ident {
                            "let" => Token::Let,
                            "print" => Token::PrintMethod,
                            _ => Token::Ident(ident.to_string()),
                        }
                    } else if self.ch.is_ascii_digit() {
                        self.read_numeral();

                        let num_str = &self.input.as_str()[self.pos..(self.next_pos)];
                        let num = num_str
                            .parse::<i64>()
                            .expect("parsing of num string failed");
                        Token::Num(num)
                    } else {
                        Token::Illegal
                    }
                }
            }
        };

        self.pos = self.next_pos;
        self.next_pos += 1;

        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_char_tokens() {
        let input = "( ) + - * / ";
        let test_case: Vec<Token> = vec![
            Token::LParen,
            Token::RParen,
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Eof,
        ];

        let mut lexer = Lexer::new(input);
        for token in test_case {
            if let Some(lexer_token) = lexer.next() {
                assert_eq!(lexer_token, token);
            }
        }
    }

    #[test]
    fn test_multi_char_tokens() {
        let input = "let yep print 10";

        let test_case = vec![
            Token::Let,
            Token::Ident("yep".into()),
            Token::PrintMethod,
            Token::Num(10),
        ];

        let mut lexer = Lexer::new(input);
        for test_case_token in test_case {
            if let Some(lexer_token) = lexer.next() {
                assert_eq!(lexer_token, test_case_token);
            }
        }
    }

    #[test]
    pub fn test_lexer() {
        let input = "
            (let ((x 10)
                 (y 6)
                 (res (+ x y)))
             print res)";

        let test_cases: Vec<Token> = vec![
            Token::LParen,
            Token::Let,
            Token::LParen,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Num(10),
            Token::RParen,
            Token::LParen,
            Token::Ident(String::from("y")),
            Token::Num(6),
            Token::RParen,
            Token::LParen,
            Token::Ident(String::from("res")),
            Token::LParen,
            Token::Plus,
            Token::Ident(String::from("x")),
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::RParen,
            Token::RParen,
            Token::PrintMethod,
            Token::Ident(String::from("res")),
            Token::RParen,
            Token::Eof,
        ];

        // let input = "(let (x 10)))";

        let mut lexer = Lexer::new(input);
        for test in test_cases {
            match lexer.next() {
                Some(token) => assert_eq!(token, test),
                _ => (),
            }
        }

        let test_case = vec![
            Token::LParen,
            Token::Let,
            Token::LParen,
            Token::LParen,
            Token::Ident("n".to_string()),
            Token::RParen,
            Token::RParen,
            Token::PrintMethod,
            Token::Ident("n".to_string()),
            Token::RParen,
        ];
        let input = "(let ((n 2)) print n)";
        let mut lexer = Lexer::new(input);
    }
}
