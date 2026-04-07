use crate::ast::{
    ArrayLiteralExpr, BlockStatement, BooleanLiteral, CallExpression, Expression,
    ExpressionStatement, ForExpression, FunctionLiteral, HashLiteralExpr, Identifier,
    IfExpression, IndexExpression, InfixExpression, IntegerLiteral, LetStatement,
    PrefixExpression, Program, ReturnStatement, Statement, StringLiteralExpr, WhileExpression,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Assign,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    cur_token: Token,
    peek_token: Token,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            cur_token: Token::new(TokenType::Illegal, ""),
            peek_token: Token::new(TokenType::Illegal, ""),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.register_prefix(TokenType::Ident, Parser::parse_identifier);
        parser.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        parser.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        parser.register_prefix(TokenType::True, Parser::parse_boolean);
        parser.register_prefix(TokenType::False, Parser::parse_boolean);
        parser.register_prefix(TokenType::LParen, Parser::parse_grouped_expression);
        parser.register_prefix(TokenType::If, Parser::parse_if_expression);
        parser.register_prefix(TokenType::Function, Parser::parse_function_literal);
        parser.register_prefix(TokenType::StringLiteral, Parser::parse_string_literal);
        parser.register_prefix(TokenType::LBracket, Parser::parse_array_literal);
        parser.register_prefix(TokenType::LBrace, Parser::parse_hash_literal);
        parser.register_prefix(TokenType::While, Parser::parse_while_expression);
        parser.register_prefix(TokenType::For, Parser::parse_for_expression);

        parser.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Eq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::NotEq, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Lt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::Gt, Parser::parse_infix_expression);
        parser.register_infix(TokenType::LParen, Parser::parse_call_expression);
        parser.register_infix(TokenType::LBracket, Parser::parse_index_expression);
        parser.register_infix(TokenType::Assign, Parser::parse_infix_expression);

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::default();

        while !self.cur_token_is(TokenType::Eof) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        program
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement().map(Statement::Let),
            TokenType::Return => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(LetStatement { token, name, value })
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.cur_token.clone();

        self.next_token();
        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(ReturnStatement {
            token,
            return_value,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(ExpressionStatement { token, expression })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = match self
            .prefix_parse_fns
            .get(&self.cur_token.token_type)
            .copied()
        {
            Some(prefix) => prefix,
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.token_type);
                return None;
            }
        };

        let mut left_expression = prefix(self)?;

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match self
                .infix_parse_fns
                .get(&self.peek_token.token_type)
                .copied()
            {
                Some(infix) => infix,
                None => return Some(left_expression),
            };

            self.next_token();
            left_expression = infix(self, left_expression)?;
        }

        Some(left_expression)
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        match self.cur_token.literal.parse::<i64>() {
            Ok(value) => Some(Expression::Integer(IntegerLiteral {
                token: self.cur_token.clone(),
                value,
            })),
            Err(_) => {
                self.errors.push(format!(
                    "could not parse {:?} as integer",
                    self.cur_token.literal
                ));
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();

        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        Some(Expression::Prefix(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();
        let right = self.parse_expression(precedence)?;

        Some(Expression::Infix(InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(BooleanLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let expression = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(expression)
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token_is(TokenType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Expression::If(IfExpression {
            token,
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::Function(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();
        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        Some(Expression::Call(CallExpression {
            token: self.cur_token.clone(),
            function: Box::new(function),
            arguments: self.parse_expression_list(TokenType::RParen)?,
        }))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Option<Vec<Expression>> {
        let mut list = Vec::new();

        if self.peek_token_is(end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteralExpr {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        Some(Expression::ArrayLiteral(ArrayLiteralExpr {
            token: self.cur_token.clone(),
            elements: self.parse_expression_list(TokenType::RBracket)?,
        }))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        let token = self.cur_token.clone();
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RBracket) {
            return None;
        }

        Some(Expression::Index(IndexExpression {
            token,
            left: Box::new(left),
            index: Box::new(index),
        }))
    }

    fn parse_hash_literal(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();
        let mut pairs = Vec::new();

        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();
            let key = self.parse_expression(Precedence::Lowest)?;

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest)?;
            pairs.push((key, value));

            if !self.peek_token_is(TokenType::RBrace) && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peek(TokenType::RBrace) {
            return None;
        }

        Some(Expression::HashLiteral(HashLiteralExpr { token, pairs }))
    }

    fn parse_while_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::While(WhileExpression {
            token,
            condition: Box::new(condition),
            body,
        }))
    }

    fn parse_for_expression(&mut self) -> Option<Expression> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        self.next_token();
        let init = self.parse_statement()?;

        // semicolon after init is consumed by parse_statement
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::Semicolon) {
            return None;
        }

        self.next_token();
        let update = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::For(ForExpression {
            token,
            init: Box::new(init),
            condition: Box::new(condition),
            update: Box::new(update),
            body,
        }))
    }

    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        ));
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "no prefix parse function for {:?} found",
            token_type
        ));
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_for(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        precedence_for(self.cur_token.token_type)
    }
}

fn precedence_for(token_type: TokenType) -> Precedence {
    match token_type {
        TokenType::Assign => Precedence::Assign,
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Slash | TokenType::Asterisk => Precedence::Product,
        TokenType::LParen => Precedence::Call,
        TokenType::LBracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
