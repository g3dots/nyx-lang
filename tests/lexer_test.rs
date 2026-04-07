use nyx::lexer::Lexer;
use nyx::token::TokenType;

#[test]
fn test_next_token() {
    let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
[1, 2];
{"foo": "bar"}
while (x < 10) {}
for (let i = 0; i < 10; i) {}
"#;

    let tests: Vec<(TokenType, &str)> = vec![
        (TokenType::Let, "let"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "ten"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "add"),
        (TokenType::Assign, "="),
        (TokenType::Function, "fn"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::RParen, ")"),
        (TokenType::Semicolon, ";"),
        (TokenType::Bang, "!"),
        (TokenType::Minus, "-"),
        (TokenType::Slash, "/"),
        (TokenType::Asterisk, "*"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::Gt, ">"),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::If, "if"),
        (TokenType::LParen, "("),
        (TokenType::Int, "5"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::True, "true"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Else, "else"),
        (TokenType::LBrace, "{"),
        (TokenType::Return, "return"),
        (TokenType::False, "false"),
        (TokenType::Semicolon, ";"),
        (TokenType::RBrace, "}"),
        (TokenType::Int, "10"),
        (TokenType::Eq, "=="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Int, "10"),
        (TokenType::NotEq, "!="),
        (TokenType::Int, "9"),
        (TokenType::Semicolon, ";"),
        // strings
        (TokenType::StringLiteral, "foobar"),
        (TokenType::StringLiteral, "foo bar"),
        // arrays
        (TokenType::LBracket, "["),
        (TokenType::Int, "1"),
        (TokenType::Comma, ","),
        (TokenType::Int, "2"),
        (TokenType::RBracket, "]"),
        (TokenType::Semicolon, ";"),
        // hashes
        (TokenType::LBrace, "{"),
        (TokenType::StringLiteral, "foo"),
        (TokenType::Colon, ":"),
        (TokenType::StringLiteral, "bar"),
        (TokenType::RBrace, "}"),
        // while
        (TokenType::While, "while"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::RBrace, "}"),
        // for
        (TokenType::For, "for"),
        (TokenType::LParen, "("),
        (TokenType::Let, "let"),
        (TokenType::Ident, "i"),
        (TokenType::Assign, "="),
        (TokenType::Int, "0"),
        (TokenType::Semicolon, ";"),
        (TokenType::Ident, "i"),
        (TokenType::Lt, "<"),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Ident, "i"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::RBrace, "}"),
        (TokenType::Eof, ""),
    ];

    let mut l = Lexer::new(input);

    for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, *expected_type,
            "tests[{i}] - token_type wrong. expected={:?}, got={:?}",
            expected_type, tok.token_type
        );
        assert_eq!(
            tok.literal, *expected_literal,
            "tests[{i}] - literal wrong. expected={:?}, got={:?}",
            expected_literal, tok.literal
        );
    }
}
