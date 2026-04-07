use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::*;
use crate::environment::Environment;
use crate::object::Object;

pub fn eval(program: &Program, env: &Rc<RefCell<Environment>>) -> Object {
    eval_program(&program.statements, env)
}

fn eval_program(statements: &[Statement], env: &Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        match result {
            Object::ReturnValue(val) => return *val,
            Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

fn eval_block_statement(block: &BlockStatement, env: &Rc<RefCell<Environment>>) -> Object {
    let mut result = Object::Null;

    for statement in &block.statements {
        result = eval_statement(statement, env);

        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

fn eval_statement(statement: &Statement, env: &Rc<RefCell<Environment>>) -> Object {
    match statement {
        Statement::Let(stmt) => {
            let val = eval_expression(&stmt.value, env);
            if is_error(&val) {
                return val;
            }
            env.borrow_mut().set(stmt.name.value.clone(), val)
        }
        Statement::Return(stmt) => {
            let val = eval_expression(&stmt.return_value, env);
            if is_error(&val) {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
        Statement::Expression(stmt) => eval_expression(&stmt.expression, env),
    }
}

fn eval_expression(expression: &Expression, env: &Rc<RefCell<Environment>>) -> Object {
    match expression {
        Expression::Integer(lit) => Object::Integer(lit.value),
        Expression::Boolean(lit) => Object::Boolean(lit.value),
        Expression::StringLiteral(lit) => Object::StringObj(lit.value.clone()),

        Expression::Prefix(expr) => {
            let right = eval_expression(&expr.right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&expr.operator, right)
        }

        Expression::Infix(expr) if expr.operator == "=" => eval_assign_expression(expr, env),

        Expression::Infix(expr) => {
            let left = eval_expression(&expr.left, env);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(&expr.right, env);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&expr.operator, left, right)
        }

        Expression::If(expr) => eval_if_expression(expr, env),

        Expression::Identifier(ident) => eval_identifier(ident, env),

        Expression::Function(lit) => Object::Function {
            parameters: lit.parameters.clone(),
            body: lit.body.clone(),
            env: Rc::clone(env),
        },

        Expression::Call(expr) => {
            let function = eval_expression(&expr.function, env);
            if is_error(&function) {
                return function;
            }
            let args = eval_expressions(&expr.arguments, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args.into_iter().next().unwrap();
            }
            apply_function(function, args)
        }

        Expression::ArrayLiteral(lit) => {
            let elements = eval_expressions(&lit.elements, env);
            if elements.len() == 1 && is_error(&elements[0]) {
                return elements.into_iter().next().unwrap();
            }
            Object::Array(elements)
        }

        Expression::Index(expr) => {
            let left = eval_expression(&expr.left, env);
            if is_error(&left) {
                return left;
            }
            let index = eval_expression(&expr.index, env);
            if is_error(&index) {
                return index;
            }
            eval_index_expression(left, index)
        }

        Expression::HashLiteral(lit) => eval_hash_literal(lit, env),

        Expression::While(expr) => eval_while_expression(expr, env),
        Expression::For(expr) => eval_for_expression(expr, env),
    }
}

fn eval_assign_expression(
    expr: &InfixExpression,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let val = eval_expression(&expr.right, env);
    if is_error(&val) {
        return val;
    }

    let Expression::Identifier(ident) = &*expr.left else {
        return Object::Error("cannot assign to non-identifier".to_string());
    };

    match env.borrow_mut().update(&ident.value, val.clone()) {
        Some(_) => val,
        None => Object::Error(format!("identifier not found: {}", ident.value)),
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => Object::Error(format!(
            "unknown operator: {}{}", operator, right.type_name()
        )),
    }
}

fn eval_bang_operator(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix_operator(right: Object) -> Object {
    match right {
        Object::Integer(v) => Object::Integer(-v),
        _ => Object::Error(format!("unknown operator: -{}", right.type_name())),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => {
            eval_integer_infix_expression(operator, *l, *r)
        }
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error(format!(
                "unknown operator: {} {} {}",
                left.type_name(), operator, right.type_name()
            )),
        },
        (Object::StringObj(l), Object::StringObj(r)) => {
            eval_string_infix_expression(operator, l, r)
        }
        _ => {
            if left.type_name() != right.type_name() {
                Object::Error(format!(
                    "type mismatch: {} {} {}",
                    left.type_name(), operator, right.type_name()
                ))
            } else {
                Object::Error(format!(
                    "unknown operator: {} {} {}",
                    left.type_name(), operator, right.type_name()
                ))
            }
        }
    }
}

fn eval_integer_infix_expression(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Error(format!("unknown operator: INTEGER {operator} INTEGER")),
    }
}

fn eval_string_infix_expression(operator: &str, left: &str, right: &str) -> Object {
    match operator {
        "+" => Object::StringObj(format!("{left}{right}")),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Error(format!("unknown operator: STRING {operator} STRING")),
    }
}

fn eval_if_expression(expr: &IfExpression, env: &Rc<RefCell<Environment>>) -> Object {
    let condition = eval_expression(&expr.condition, env);
    if is_error(&condition) {
        return condition;
    }

    if is_truthy(&condition) {
        eval_block_statement(&expr.consequence, env)
    } else if let Some(alternative) = &expr.alternative {
        eval_block_statement(alternative, env)
    } else {
        Object::Null
    }
}

fn eval_identifier(ident: &Identifier, env: &Rc<RefCell<Environment>>) -> Object {
    if let Some(val) = env.borrow().get(&ident.value) {
        return val;
    }

    if let Some(builtin) = get_builtin(&ident.value) {
        return builtin;
    }

    Object::Error(format!("identifier not found: {}", ident.value))
}

fn eval_expressions(
    expressions: &[Expression],
    env: &Rc<RefCell<Environment>>,
) -> Vec<Object> {
    let mut result = Vec::new();

    for expr in expressions {
        let evaluated = eval_expression(expr, env);
        if is_error(&evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }

    result
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    match function {
        Object::Function {
            parameters,
            body,
            env,
        } => {
            if parameters.len() != args.len() {
                return Object::Error(format!(
                    "wrong number of arguments: expected {}, got {}",
                    parameters.len(),
                    args.len()
                ));
            }

            let enclosed_env = Rc::new(RefCell::new(Environment::new_enclosed(env)));
            for (param, arg) in parameters.iter().zip(args) {
                enclosed_env.borrow_mut().set(param.value.clone(), arg);
            }

            let result = eval_block_statement(&body, &enclosed_env);
            match result {
                Object::ReturnValue(val) => *val,
                other => other,
            }
        }
        Object::Builtin(_, func) => func(args),
        _ => Object::Error(format!("not a function: {}", function.type_name())),
    }
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    match (&left, &index) {
        (Object::Array(elements), Object::Integer(idx)) => {
            if *idx < 0 || *idx as usize >= elements.len() {
                Object::Null
            } else {
                elements[*idx as usize].clone()
            }
        }
        (Object::Hash(map), _) => match index.as_hash_key() {
            Some(key) => map.get(&key).cloned().unwrap_or(Object::Null),
            None => Object::Error(format!("unusable as hash key: {}", index.type_name())),
        },
        _ => Object::Error(format!(
            "index operator not supported: {}",
            left.type_name()
        )),
    }
}

fn eval_hash_literal(
    lit: &HashLiteralExpr,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let mut map = HashMap::new();

    for (key_expr, val_expr) in &lit.pairs {
        let key = eval_expression(key_expr, env);
        if is_error(&key) {
            return key;
        }

        let hash_key = match key.as_hash_key() {
            Some(k) => k,
            None => return Object::Error(format!("unusable as hash key: {}", key.type_name())),
        };

        let val = eval_expression(val_expr, env);
        if is_error(&val) {
            return val;
        }

        map.insert(hash_key, val);
    }

    Object::Hash(map)
}

fn eval_while_expression(
    expr: &WhileExpression,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let mut result = Object::Null;

    loop {
        let condition = eval_expression(&expr.condition, env);
        if is_error(&condition) {
            return condition;
        }
        if !is_truthy(&condition) {
            break;
        }

        result = eval_block_statement(&expr.body, env);
        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }
    }

    result
}

fn eval_for_expression(
    expr: &ForExpression,
    env: &Rc<RefCell<Environment>>,
) -> Object {
    let init_result = eval_statement(&expr.init, env);
    if is_error(&init_result) {
        return init_result;
    }

    let mut result = Object::Null;

    loop {
        let condition = eval_expression(&expr.condition, env);
        if is_error(&condition) {
            return condition;
        }
        if !is_truthy(&condition) {
            break;
        }

        result = eval_block_statement(&expr.body, env);
        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => {}
        }

        let update = eval_expression(&expr.update, env);
        if is_error(&update) {
            return update;
        }
    }

    result
}

// -- builtins --

fn get_builtin(name: &str) -> Option<Object> {
    match name {
        "len" => Some(Object::Builtin("len".into(), builtin_len)),
        "puts" => Some(Object::Builtin("puts".into(), builtin_puts)),
        "first" => Some(Object::Builtin("first".into(), builtin_first)),
        "last" => Some(Object::Builtin("last".into(), builtin_last)),
        "rest" => Some(Object::Builtin("rest".into(), builtin_rest)),
        "push" => Some(Object::Builtin("push".into(), builtin_push)),
        _ => None,
    }
}

fn builtin_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    match &args[0] {
        Object::StringObj(s) => Object::Integer(s.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        other => Object::Error(format!(
            "argument to `len` not supported, got {}",
            other.type_name()
        )),
    }
}

fn builtin_puts(args: Vec<Object>) -> Object {
    for arg in &args {
        println!("{arg}");
    }
    Object::Null
}

fn builtin_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(arr) => arr.first().cloned().unwrap_or(Object::Null),
        other => Object::Error(format!(
            "argument to `first` must be ARRAY, got {}",
            other.type_name()
        )),
    }
}

fn builtin_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(arr) => arr.last().cloned().unwrap_or(Object::Null),
        other => Object::Error(format!(
            "argument to `last` must be ARRAY, got {}",
            other.type_name()
        )),
    }
}

fn builtin_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Object::Null
            } else {
                Object::Array(arr[1..].to_vec())
            }
        }
        other => Object::Error(format!(
            "argument to `rest` must be ARRAY, got {}",
            other.type_name()
        )),
    }
}

fn builtin_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }
    match &args[0] {
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Object::Array(new_arr)
        }
        other => Object::Error(format!(
            "argument to `push` must be ARRAY, got {}",
            other.type_name()
        )),
    }
}

// -- helpers --

fn is_truthy(obj: &Object) -> bool {
    !matches!(obj, Object::Null | Object::Boolean(false))
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}
