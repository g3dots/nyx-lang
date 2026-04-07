use nyx::ast::{
    Expression, ExpressionStatement, FunctionLiteral, Identifier, IfExpression, Program, Statement,
    WhileExpression,
};
use nyx::lexer::Lexer;
use nyx::parser::Parser;

#[test]
fn test_let_statements() {
    let tests = [
        ("let x = 5;", "x", LiteralExpectation::Int(5)),
        ("let y = true;", "y", LiteralExpectation::Bool(true)),
        ("let foobar = y;", "foobar", LiteralExpectation::Ident("y")),
    ];

    for (input, expected_identifier, expected_value) in tests {
        let program = parse_program(input);

        assert_eq!(program.statements.len(), 1);

        let statement = &program.statements[0];
        assert!(test_let_statement(statement, expected_identifier));

        let Statement::Let(let_statement) = statement else {
            panic!("statement was not let");
        };

        assert!(test_literal_expression(
            &let_statement.value,
            expected_value
        ));
    }
}

#[test]
fn test_return_statements() {
    let tests = [
        ("return 5;", LiteralExpectation::Int(5)),
        ("return true;", LiteralExpectation::Bool(true)),
        ("return foobar;", LiteralExpectation::Ident("foobar")),
    ];

    for (input, expected_value) in tests {
        let program = parse_program(input);

        assert_eq!(program.statements.len(), 1);

        let Statement::Return(return_statement) = &program.statements[0] else {
            panic!("statement was not return");
        };

        assert_eq!(return_statement.token_literal(), "return");
        assert!(test_literal_expression(
            &return_statement.return_value,
            expected_value
        ));
    }
}

#[test]
fn test_identifier_expression() {
    let program = parse_program("foobar;");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::Identifier(identifier) = &statement.expression else {
        panic!("expression was not identifier");
    };

    assert_eq!(identifier.value, "foobar");
    assert_eq!(identifier.token_literal(), "foobar");
}

#[test]
fn test_integer_literal_expression() {
    let program = parse_program("5;");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::Integer(integer) = &statement.expression else {
        panic!("expression was not integer literal");
    };

    assert_eq!(integer.value, 5);
    assert_eq!(integer.token_literal(), "5");
}

#[test]
fn test_boolean_expression() {
    let tests = [("true;", true), ("false;", false)];

    for (input, expected_boolean) in tests {
        let program = parse_program(input);
        assert_eq!(program.statements.len(), 1);

        let statement = expect_expression_statement(&program.statements[0]);
        let Expression::Boolean(boolean) = &statement.expression else {
            panic!("expression was not boolean");
        };

        assert_eq!(boolean.value, expected_boolean);
        assert_eq!(boolean.token_literal(), expected_boolean.to_string());
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    let tests = [
        ("!5;", "!", LiteralExpectation::Int(5)),
        ("-15;", "-", LiteralExpectation::Int(15)),
        ("!foobar;", "!", LiteralExpectation::Ident("foobar")),
        ("-foobar;", "-", LiteralExpectation::Ident("foobar")),
        ("!true;", "!", LiteralExpectation::Bool(true)),
        ("!false;", "!", LiteralExpectation::Bool(false)),
    ];

    for (input, operator, value) in tests {
        let program = parse_program(input);
        assert_eq!(program.statements.len(), 1);

        let statement = expect_expression_statement(&program.statements[0]);
        let Expression::Prefix(prefix_expression) = &statement.expression else {
            panic!("expression was not prefix");
        };

        assert_eq!(prefix_expression.operator, operator);
        assert!(test_literal_expression(&prefix_expression.right, value));
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let tests = [
        (
            "5 + 5;",
            LiteralExpectation::Int(5),
            "+",
            LiteralExpectation::Int(5),
        ),
        (
            "5 - 5;",
            LiteralExpectation::Int(5),
            "-",
            LiteralExpectation::Int(5),
        ),
        (
            "5 * 5;",
            LiteralExpectation::Int(5),
            "*",
            LiteralExpectation::Int(5),
        ),
        (
            "5 / 5;",
            LiteralExpectation::Int(5),
            "/",
            LiteralExpectation::Int(5),
        ),
        (
            "5 > 5;",
            LiteralExpectation::Int(5),
            ">",
            LiteralExpectation::Int(5),
        ),
        (
            "5 < 5;",
            LiteralExpectation::Int(5),
            "<",
            LiteralExpectation::Int(5),
        ),
        (
            "5 == 5;",
            LiteralExpectation::Int(5),
            "==",
            LiteralExpectation::Int(5),
        ),
        (
            "5 != 5;",
            LiteralExpectation::Int(5),
            "!=",
            LiteralExpectation::Int(5),
        ),
        (
            "foobar + barfoo;",
            LiteralExpectation::Ident("foobar"),
            "+",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar - barfoo;",
            LiteralExpectation::Ident("foobar"),
            "-",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar * barfoo;",
            LiteralExpectation::Ident("foobar"),
            "*",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar / barfoo;",
            LiteralExpectation::Ident("foobar"),
            "/",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar > barfoo;",
            LiteralExpectation::Ident("foobar"),
            ">",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar < barfoo;",
            LiteralExpectation::Ident("foobar"),
            "<",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar == barfoo;",
            LiteralExpectation::Ident("foobar"),
            "==",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "foobar != barfoo;",
            LiteralExpectation::Ident("foobar"),
            "!=",
            LiteralExpectation::Ident("barfoo"),
        ),
        (
            "true == true",
            LiteralExpectation::Bool(true),
            "==",
            LiteralExpectation::Bool(true),
        ),
        (
            "true != false",
            LiteralExpectation::Bool(true),
            "!=",
            LiteralExpectation::Bool(false),
        ),
        (
            "false == false",
            LiteralExpectation::Bool(false),
            "==",
            LiteralExpectation::Bool(false),
        ),
    ];

    for (input, left_value, operator, right_value) in tests {
        let program = parse_program(input);
        assert_eq!(program.statements.len(), 1);

        let statement = expect_expression_statement(&program.statements[0]);
        assert!(test_infix_expression(
            &statement.expression,
            left_value,
            operator,
            right_value
        ));
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = [
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
    ];

    for (input, expected) in tests {
        let program = parse_program(input);
        assert_eq!(program.to_string(), expected);
    }
}

#[test]
fn test_if_expression() {
    let program = parse_program("if (x < y) { x }");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::If(if_expression) = &statement.expression else {
        panic!("expression was not if");
    };

    assert!(test_infix_expression(
        &if_expression.condition,
        LiteralExpectation::Ident("x"),
        "<",
        LiteralExpectation::Ident("y")
    ));
    assert_eq!(if_expression.consequence.statements.len(), 1);

    let consequence = expect_expression_statement(&if_expression.consequence.statements[0]);
    assert!(test_identifier(&consequence.expression, "x"));
    assert!(if_expression.alternative.is_none());
}

#[test]
fn test_if_else_expression() {
    let program = parse_program("if (x < y) { x } else { y }");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::If(if_expression) = &statement.expression else {
        panic!("expression was not if");
    };

    assert!(test_infix_expression(
        &if_expression.condition,
        LiteralExpectation::Ident("x"),
        "<",
        LiteralExpectation::Ident("y")
    ));
    assert_eq!(if_expression.consequence.statements.len(), 1);

    let consequence = expect_expression_statement(&if_expression.consequence.statements[0]);
    assert!(test_identifier(&consequence.expression, "x"));

    let alternative = if_expression
        .alternative
        .as_ref()
        .expect("if expression was missing else block");
    assert_eq!(alternative.statements.len(), 1);

    let alternative_statement = expect_expression_statement(&alternative.statements[0]);
    assert!(test_identifier(&alternative_statement.expression, "y"));
}

#[test]
fn test_function_literal_parsing() {
    let program = parse_program("fn(x, y) { x + y; }");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::Function(function) = &statement.expression else {
        panic!("expression was not function literal");
    };

    assert_eq!(function.parameters.len(), 2);
    assert!(test_literal_expression(
        &Expression::Identifier(function.parameters[0].clone()),
        LiteralExpectation::Ident("x")
    ));
    assert!(test_literal_expression(
        &Expression::Identifier(function.parameters[1].clone()),
        LiteralExpectation::Ident("y")
    ));
    assert_eq!(function.body.statements.len(), 1);

    let body_statement = expect_expression_statement(&function.body.statements[0]);
    assert!(test_infix_expression(
        &body_statement.expression,
        LiteralExpectation::Ident("x"),
        "+",
        LiteralExpectation::Ident("y")
    ));
}

#[test]
fn test_function_parameter_parsing() {
    let tests = [
        ("fn() {};", Vec::<&str>::new()),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected_params) in tests {
        let program = parse_program(input);
        let statement = expect_expression_statement(&program.statements[0]);
        let Expression::Function(function) = &statement.expression else {
            panic!("expression was not function literal");
        };

        assert_eq!(function.parameters.len(), expected_params.len());
        for (identifier, expected) in function.parameters.iter().zip(expected_params.iter()) {
            assert!(test_literal_expression(
                &Expression::Identifier(identifier.clone()),
                LiteralExpectation::Ident(expected)
            ));
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let program = parse_program("add(1, 2 * 3, 4 + 5);");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::Call(call_expression) = &statement.expression else {
        panic!("expression was not call expression");
    };

    assert!(test_identifier(&call_expression.function, "add"));
    assert_eq!(call_expression.arguments.len(), 3);
    assert!(test_literal_expression(
        &call_expression.arguments[0],
        LiteralExpectation::Int(1)
    ));
    assert!(test_infix_expression(
        &call_expression.arguments[1],
        LiteralExpectation::Int(2),
        "*",
        LiteralExpectation::Int(3)
    ));
    assert!(test_infix_expression(
        &call_expression.arguments[2],
        LiteralExpectation::Int(4),
        "+",
        LiteralExpectation::Int(5)
    ));
}

#[test]
fn test_call_expression_parameter_parsing() {
    let tests = [
        ("add();", "add", Vec::<&str>::new()),
        ("add(1);", "add", vec!["1"]),
        (
            "add(1, 2 * 3, 4 + 5);",
            "add",
            vec!["1", "(2 * 3)", "(4 + 5)"],
        ),
    ];

    for (input, expected_ident, expected_args) in tests {
        let program = parse_program(input);
        let statement = expect_expression_statement(&program.statements[0]);
        let Expression::Call(call_expression) = &statement.expression else {
            panic!("expression was not call expression");
        };

        assert!(test_identifier(&call_expression.function, expected_ident));
        assert_eq!(call_expression.arguments.len(), expected_args.len());

        for (argument, expected) in call_expression.arguments.iter().zip(expected_args.iter()) {
            assert_eq!(argument.to_string(), *expected);
        }
    }
}

#[derive(Clone, Copy)]
enum LiteralExpectation<'a> {
    Int(i64),
    Ident(&'a str),
    Bool(bool),
}

fn parse_program(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    program
}

fn expect_expression_statement(statement: &Statement) -> &ExpressionStatement {
    let Statement::Expression(expression_statement) = statement else {
        panic!("statement was not expression statement: {statement:?}");
    };

    expression_statement
}

fn test_let_statement(statement: &Statement, name: &str) -> bool {
    if statement.token_literal() != "let" {
        return false;
    }

    let Statement::Let(let_statement) = statement else {
        return false;
    };

    let_statement.name.value == name && let_statement.name.token_literal() == name
}

fn test_infix_expression(
    expression: &Expression,
    left: LiteralExpectation<'_>,
    operator: &str,
    right: LiteralExpectation<'_>,
) -> bool {
    let Expression::Infix(infix_expression) = expression else {
        return false;
    };

    test_literal_expression(&infix_expression.left, left)
        && infix_expression.operator == operator
        && test_literal_expression(&infix_expression.right, right)
}

fn test_literal_expression(expression: &Expression, expected: LiteralExpectation<'_>) -> bool {
    match expected {
        LiteralExpectation::Int(value) => test_integer_literal(expression, value),
        LiteralExpectation::Ident(value) => test_identifier(expression, value),
        LiteralExpectation::Bool(value) => test_boolean_literal(expression, value),
    }
}

fn test_integer_literal(expression: &Expression, value: i64) -> bool {
    let Expression::Integer(integer) = expression else {
        return false;
    };

    integer.value == value && integer.token_literal() == value.to_string()
}

fn test_identifier(expression: &Expression, value: &str) -> bool {
    let Expression::Identifier(identifier) = expression else {
        return false;
    };

    identifier.value == value && identifier.token_literal() == value
}

fn test_boolean_literal(expression: &Expression, value: bool) -> bool {
    let Expression::Boolean(boolean) = expression else {
        return false;
    };

    boolean.value == value && boolean.token_literal() == value.to_string()
}

fn check_parser_errors(parser: &Parser) {
    if parser.errors().is_empty() {
        return;
    }

    panic!("parser had errors: {:?}", parser.errors());
}

#[test]
fn test_string_literal_expression() {
    let program = parse_program(r#""hello world";"#);
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::StringLiteral(string_lit) = &statement.expression else {
        panic!("expression was not string literal");
    };
    assert_eq!(string_lit.value, "hello world");
}

#[test]
fn test_array_literal_parsing() {
    let program = parse_program("[1, 2 * 2, 3 + 3]");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::ArrayLiteral(array) = &statement.expression else {
        panic!("expression was not array literal");
    };
    assert_eq!(array.elements.len(), 3);
    assert!(test_integer_literal(&array.elements[0], 1));
    assert!(test_infix_expression(
        &array.elements[1],
        LiteralExpectation::Int(2),
        "*",
        LiteralExpectation::Int(2),
    ));
    assert!(test_infix_expression(
        &array.elements[2],
        LiteralExpectation::Int(3),
        "+",
        LiteralExpectation::Int(3),
    ));
}

#[test]
fn test_index_expression_parsing() {
    let program = parse_program("myArray[1 + 1]");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::Index(index_expr) = &statement.expression else {
        panic!("expression was not index expression");
    };
    assert!(test_identifier(&index_expr.left, "myArray"));
    assert!(test_infix_expression(
        &index_expr.index,
        LiteralExpectation::Int(1),
        "+",
        LiteralExpectation::Int(1),
    ));
}

#[test]
fn test_hash_literal_parsing() {
    let program = parse_program(r#"{"one": 1, "two": 2, "three": 3}"#);
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::HashLiteral(hash) = &statement.expression else {
        panic!("expression was not hash literal");
    };
    assert_eq!(hash.pairs.len(), 3);
}

#[test]
fn test_empty_hash_literal() {
    let program = parse_program("{}");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::HashLiteral(hash) = &statement.expression else {
        panic!("expression was not hash literal");
    };
    assert_eq!(hash.pairs.len(), 0);
}

#[test]
fn test_while_expression_parsing() {
    let program = parse_program("while (x < 10) { x }");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::While(while_expr) = &statement.expression else {
        panic!("expression was not while");
    };

    assert!(test_infix_expression(
        &while_expr.condition,
        LiteralExpectation::Ident("x"),
        "<",
        LiteralExpectation::Int(10),
    ));
    assert_eq!(while_expr.body.statements.len(), 1);
}

#[test]
fn test_for_expression_parsing() {
    let program = parse_program("for (let i = 0; i < 10; i = i + 1) { x }");
    assert_eq!(program.statements.len(), 1);

    let statement = expect_expression_statement(&program.statements[0]);
    let Expression::For(for_expr) = &statement.expression else {
        panic!("expression was not for");
    };

    let Statement::Let(init) = &*for_expr.init else {
        panic!("for init was not let statement");
    };
    assert_eq!(init.name.value, "i");

    assert!(test_infix_expression(
        &for_expr.condition,
        LiteralExpectation::Ident("i"),
        "<",
        LiteralExpectation::Int(10),
    ));

    assert_eq!(for_expr.body.statements.len(), 1);
}

#[test]
fn test_operator_precedence_with_index() {
    let tests = [
        ("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        ("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
    ];

    for (input, expected) in tests {
        let program = parse_program(input);
        assert_eq!(program.to_string(), expected);
    }
}

#[allow(dead_code)]
fn _assert_ast_types(_: &Identifier, _: &IfExpression, _: &FunctionLiteral, _: &WhileExpression) {}
