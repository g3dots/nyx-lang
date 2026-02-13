use crate::token::{Token, TokenType, lookup_ident};

pub struct Lexer {
    input: Vec<u8>,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    Token::new(TokenType::Eq, literal)
                } else {
                    Token::new(TokenType::Assign, "=")
                }
            }
            b'+' => Token::new(TokenType::Plus, "+"),
            b'-' => Token::new(TokenType::Minus, "-"),
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    Token::new(TokenType::NotEq, literal)
                } else {
                    Token::new(TokenType::Bang, "!")
                }
            }
            b'/' => Token::new(TokenType::Slash, "/"),
            b'*' => Token::new(TokenType::Asterisk, "*"),
            b'<' => Token::new(TokenType::Lt, "<"),
            b'>' => Token::new(TokenType::Gt, ">"),
            b';' => Token::new(TokenType::Semicolon, ";"),
            b',' => Token::new(TokenType::Comma, ","),
            b'{' => Token::new(TokenType::LBrace, "{"),
            b'}' => Token::new(TokenType::RBrace, "}"),
            b'(' => Token::new(TokenType::LParen, "("),
            b')' => Token::new(TokenType::RParen, ")"),
            0 => Token::new(TokenType::Eof, ""),
            ch if is_letter(ch) => {
                let literal = self.read_identifier();
                let token_type = lookup_ident(&literal);
                return Token::new(token_type, literal);
            }
            ch if is_digit(ch) => {
                let literal = self.read_number();
                return Token::new(TokenType::Int, literal);
            }
            ch => Token::new(TokenType::Illegal, (ch as char).to_string()),
        };

        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8(self.input[start..self.position].to_vec()).unwrap()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8(self.input[start..self.position].to_vec()).unwrap()
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}
