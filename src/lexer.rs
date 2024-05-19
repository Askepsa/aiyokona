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

                        let num_str = &self.input.as_str()[self.pos..(self.next_pos)];
                        let num = num_str
                            .parse::<i64>()
                            .expect("parsing of negative num string failed");
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
