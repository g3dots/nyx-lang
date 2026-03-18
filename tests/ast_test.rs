use nyx::ast::{Expression, Identifier, LetStatement, Program, Statement};
use nyx::token::{Token, TokenType};

#[test]
fn test_program_string() {
    let program = Program {
        statements: vec![Statement::Let(LetStatement {
            token: Token::new(TokenType::Let, "let"),
            name: Identifier {
                token: Token::new(TokenType::Ident, "myVar"),
                value: "myVar".to_string(),
            },
            value: Expression::Identifier(Identifier {
                token: Token::new(TokenType::Ident, "anotherVar"),
                value: "anotherVar".to_string(),
            }),
        })],
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;");
}
