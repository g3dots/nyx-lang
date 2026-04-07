use crate::environment::Environment;
use crate::evaluator;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use std::cell::RefCell;
use std::io::{self, BufRead, Write};
use std::rc::Rc;

const PROMPT: &str = ">> ";
const NYX_BANNER: &str = ".:: nyx ::.";

pub fn start() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let env = Rc::new(RefCell::new(Environment::new()));

    loop {
        print!("{}", PROMPT);
        stdout.lock().flush().expect("failed to flush stdout");

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => return,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {e}");
                return;
            }
        }

        let lexer = Lexer::new(line.trim_end());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors().is_empty() {
            print_parser_errors(parser.errors());
            continue;
        }

        let evaluated = evaluator::eval(&program, &env);
        if !matches!(evaluated, Object::Null) {
            println!("{evaluated}");
        }
    }
}

fn print_parser_errors(errors: &[String]) {
    eprintln!("{NYX_BANNER}");
    eprintln!("Woops! We ran into some Nyx parser trouble here!");
    eprintln!(" parser errors:");
    for error in errors {
        eprintln!("\t{error}");
    }
}
