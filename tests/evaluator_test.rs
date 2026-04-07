use std::cell::RefCell;
use std::rc::Rc;

use nyx::environment::Environment;
use nyx::evaluator;
use nyx::lexer::Lexer;
use nyx::object::Object;
use nyx::parser::Parser;

fn test_eval(input: &str) -> Object {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    assert!(parser.errors().is_empty(), "parser errors: {:?}", parser.errors());
    let env = Rc::new(RefCell::new(Environment::new()));
    evaluator::eval(&program, &env)
}

fn test_integer_object(obj: &Object, expected: i64) {
    match obj {
        Object::Integer(v) => assert_eq!(*v, expected, "integer value mismatch"),
        other => panic!("expected Integer, got {other}"),
    }
}

fn test_boolean_object(obj: &Object, expected: bool) {
    match obj {
        Object::Boolean(v) => assert_eq!(*v, expected, "boolean value mismatch"),
        other => panic!("expected Boolean, got {other}"),
    }
}

fn test_null_object(obj: &Object) {
    assert!(matches!(obj, Object::Null), "expected Null, got {obj}");
}

#[test]
fn test_eval_integer_expression() {
    let tests = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_eval_boolean_expression() {
    let tests = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in tests {
        test_boolean_object(&test_eval(input), expected);
    }
}

#[test]
fn test_bang_operator() {
    let tests = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        test_boolean_object(&test_eval(input), expected);
    }
}

#[test]
fn test_if_else_expressions() {
    let tests: Vec<(&str, Box<dyn Fn(&Object)>)> = vec![
        ("if (true) { 10 }", Box::new(|o| test_integer_object(o, 10))),
        ("if (false) { 10 }", Box::new(|o| test_null_object(o))),
        ("if (1) { 10 }", Box::new(|o| test_integer_object(o, 10))),
        ("if (1 < 2) { 10 }", Box::new(|o| test_integer_object(o, 10))),
        ("if (1 > 2) { 10 }", Box::new(|o| test_null_object(o))),
        ("if (1 > 2) { 10 } else { 20 }", Box::new(|o| test_integer_object(o, 20))),
        ("if (1 < 2) { 10 } else { 20 }", Box::new(|o| test_integer_object(o, 10))),
    ];

    for (input, check) in &tests {
        check(&test_eval(input));
    }
}

#[test]
fn test_return_statements() {
    let tests = [
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        ("if (10 > 1) { if (10 > 1) { return 10; } return 1; }", 10),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_error_handling() {
    let tests = [
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        ("if (10 > 1) { true + false; }", "unknown operator: BOOLEAN + BOOLEAN"),
        ("foobar", "identifier not found: foobar"),
    ];

    for (input, expected_msg) in tests {
        let result = test_eval(input);
        let Object::Error(msg) = result else {
            panic!("expected error for '{input}', got {result:?}");
        };
        assert_eq!(msg, expected_msg, "wrong error message for '{input}'");
    }
}

#[test]
fn test_let_statements() {
    let tests = [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_function_object() {
    let result = test_eval("fn(x) { x + 2; };");
    let Object::Function { parameters, body, .. } = &result else {
        panic!("expected Function, got {result:?}");
    };
    assert_eq!(parameters.len(), 1);
    assert_eq!(parameters[0].value, "x");
    assert_eq!(body.to_string(), "(x + 2)");
}

#[test]
fn test_function_application() {
    let tests = [
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_closures() {
    let input = "let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);";
    test_integer_object(&test_eval(input), 4);
}

#[test]
fn test_string_literal() {
    let result = test_eval(r#""Hello World!""#);
    match &result {
        Object::StringObj(s) => assert_eq!(s, "Hello World!"),
        other => panic!("expected StringObj, got {other:?}"),
    }
}

#[test]
fn test_string_concatenation() {
    let result = test_eval(r#""Hello" + " " + "World!""#);
    match &result {
        Object::StringObj(s) => assert_eq!(s, "Hello World!"),
        other => panic!("expected StringObj, got {other:?}"),
    }
}

#[test]
fn test_string_comparison() {
    test_boolean_object(&test_eval(r#""abc" == "abc""#), true);
    test_boolean_object(&test_eval(r#""abc" != "def""#), true);
    test_boolean_object(&test_eval(r#""abc" == "def""#), false);
}

#[test]
fn test_builtin_len() {
    let tests: Vec<(&str, Box<dyn Fn(&Object)>)> = vec![
        (r#"len("")"#, Box::new(|o| test_integer_object(o, 0))),
        (r#"len("four")"#, Box::new(|o| test_integer_object(o, 4))),
        (r#"len("hello world")"#, Box::new(|o| test_integer_object(o, 11))),
        ("len([1, 2, 3])", Box::new(|o| test_integer_object(o, 3))),
        ("len([])", Box::new(|o| test_integer_object(o, 0))),
    ];

    for (input, check) in &tests {
        check(&test_eval(input));
    }
}

#[test]
fn test_array_literals() {
    let result = test_eval("[1, 2 * 2, 3 + 3]");
    let Object::Array(elements) = &result else {
        panic!("expected Array, got {result:?}");
    };
    assert_eq!(elements.len(), 3);
    test_integer_object(&elements[0], 1);
    test_integer_object(&elements[1], 4);
    test_integer_object(&elements[2], 6);
}

#[test]
fn test_array_index_expressions() {
    let tests: Vec<(&str, Box<dyn Fn(&Object)>)> = vec![
        ("[1, 2, 3][0]", Box::new(|o| test_integer_object(o, 1))),
        ("[1, 2, 3][1]", Box::new(|o| test_integer_object(o, 2))),
        ("[1, 2, 3][2]", Box::new(|o| test_integer_object(o, 3))),
        ("let i = 0; [1][i];", Box::new(|o| test_integer_object(o, 1))),
        ("[1, 2, 3][1 + 1];", Box::new(|o| test_integer_object(o, 3))),
        ("let myArray = [1, 2, 3]; myArray[2];", Box::new(|o| test_integer_object(o, 3))),
        ("[1, 2, 3][3]", Box::new(|o| test_null_object(o))),
        ("[1, 2, 3][-1]", Box::new(|o| test_null_object(o))),
    ];

    for (input, check) in &tests {
        check(&test_eval(input));
    }
}

#[test]
fn test_hash_literals() {
    let result = test_eval(r#"let two = "two"; {"one": 10 - 9, two: 1 + 1, "thr" + "ee": 6 / 2, 4: 4, true: 5, false: 6}"#);
    let Object::Hash(map) = &result else {
        panic!("expected Hash, got {result:?}");
    };
    assert_eq!(map.len(), 6);
}

#[test]
fn test_hash_index_expressions() {
    let tests: Vec<(&str, Box<dyn Fn(&Object)>)> = vec![
        (r#"{"foo": 5}["foo"]"#, Box::new(|o| test_integer_object(o, 5))),
        (r#"{"foo": 5}["bar"]"#, Box::new(|o| test_null_object(o))),
        (r#"let key = "foo"; {"foo": 5}[key]"#, Box::new(|o| test_integer_object(o, 5))),
        (r#"{}["foo"]"#, Box::new(|o| test_null_object(o))),
        ("{5: 5}[5]", Box::new(|o| test_integer_object(o, 5))),
        ("{true: 5}[true]", Box::new(|o| test_integer_object(o, 5))),
        ("{false: 5}[false]", Box::new(|o| test_integer_object(o, 5))),
    ];

    for (input, check) in &tests {
        check(&test_eval(input));
    }
}

#[test]
fn test_builtin_first_last_rest_push() {
    test_integer_object(&test_eval("first([1, 2, 3])"), 1);
    test_integer_object(&test_eval("last([1, 2, 3])"), 3);

    let rest = test_eval("rest([1, 2, 3])");
    let Object::Array(elements) = &rest else {
        panic!("expected Array, got {rest:?}");
    };
    assert_eq!(elements.len(), 2);
    test_integer_object(&elements[0], 2);
    test_integer_object(&elements[1], 3);

    let pushed = test_eval("push([1, 2], 3)");
    let Object::Array(elements) = &pushed else {
        panic!("expected Array, got {pushed:?}");
    };
    assert_eq!(elements.len(), 3);
    test_integer_object(&elements[2], 3);
}

#[test]
fn test_assignment_expression() {
    let tests = [
        ("let x = 5; x = 10; x;", 10),
        ("let x = 5; x = x + 1; x;", 6),
        ("let a = 1; let b = 2; a = b; a;", 2),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_while_expression() {
    let tests = [
        ("let x = 0; while (x < 5) { x = x + 1; }; x;", 5),
        ("let x = 10; while (x > 0) { x = x - 2; }; x;", 0),
        ("let x = 0; while (false) { x = x + 1; }; x;", 0),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_for_expression() {
    let tests = [
        ("let sum = 0; for (let i = 0; i < 5; i = i + 1) { sum = sum + i; }; sum;", 10),
        ("let sum = 0; for (let i = 1; i < 4; i = i + 1) { sum = sum + i; }; sum;", 6),
        ("let x = 1; for (let i = 0; i < 3; i = i + 1) { x = x * 2; }; x;", 8),
    ];

    for (input, expected) in tests {
        test_integer_object(&test_eval(input), expected);
    }
}

#[test]
fn test_recursive_fibonacci() {
    let input = "
        let fib = fn(n) {
            if (n < 2) { return n; }
            fib(n - 1) + fib(n - 2);
        };
        fib(10);
    ";
    test_integer_object(&test_eval(input), 55);
}

#[test]
fn test_higher_order_functions() {
    let input = "
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), push(accumulated, f(first(arr))))
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);
    ";
    let result = test_eval(input);
    let Object::Array(elements) = &result else {
        panic!("expected Array, got {result:?}");
    };
    assert_eq!(elements.len(), 4);
    test_integer_object(&elements[0], 2);
    test_integer_object(&elements[1], 4);
    test_integer_object(&elements[2], 6);
    test_integer_object(&elements[3], 8);
}
