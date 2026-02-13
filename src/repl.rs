use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        print!("{}", PROMPT);
        stdout.lock().flush().expect("failed to flush stdout");

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => return, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {e}");
                return;
            }
        }

        let mut l = Lexer::new(line.trim_end());

        loop {
            let tok = l.next_token();
            if tok.token_type == TokenType::Eof {
                break;
            }
            println!("{tok:?}");
        }
    }
}
