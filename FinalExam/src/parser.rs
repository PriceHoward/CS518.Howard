use crate::ast::{Expr, Identifier, Program, Statement};
use crate::enums::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn check_current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume_current(&mut self) -> Option<&Token> {
        let current_token = self.tokens.get(self.position);
        self.position += 1;
        current_token
    }

    fn expect_match(&mut self, expected: &Token)-> Result<(), String> {
        match self.consume_current() {
            Some(current_token) if current_token == expected => Ok (()),
            Some(tok) => Err(format!("Expected {:?} but got {:?}", expected, tok)), // Had some AI Help with the error code items here.
            None => Err(format!("Expected {:?} but got end of input", expected)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.consume_current() {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::Ident(name)) => Ok(Expr::Variable(Identifier(name.clone()))),
            Some(tok) => Err(format!("Expected a value but got {:?}", tok)),
            None => Err("Expected a value but got end of input".to_string()),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.consume_current() {
            Some(Token::Forward) => {
                let expr = self.parse_expr()?;
                Ok(Statement::Forward(expr))
            }

            Some(Token::Turn) => {
                let expr = self.parse_expr()?;
                Ok(Statement::Turn(expr))
            }

            Some(Token::Pen) => {
                let expr = self.parse_expr()?;
                Ok(Statement::Pen(expr))
            }

            Some(Token::Set) => {
                let name = match self.consume_current() {
                    Some(Token::Ident(s)) => Identifier(s.clone()),
                    Some(tok) => return Err(format!("Expected identifier after 'set', got {:?}", tok)),
                    None => return Err("Expected identifier after 'set'".to_string()),
                };
                let value = self.parse_expr()?;
                Ok(Statement::Set { name, value })
            }

            Some(Token::Dotimes) => {
                let count = self.parse_expr()?;
                self.expect_match(&Token::LBrace)?;
                let body = self.parse_statement_list()?;
                self.expect_match(&Token::RBrace)?;
                Ok(Statement::Dotimes { count, body })
            }

            Some(tok) => Err(format!("Unexpected token {:?}", tok)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_statement_list(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        loop {
            match self.check_current() {
                None | Some(Token::RBrace) => break,
                _ => statements.push(self.parse_statement()?),
            }
        }

        Ok(statements)
    }

    pub fn parse(mut self) -> Result<Program, String> {
        let statements = self.parse_statement_list()?;
        Ok(Program { statements })
    }
}
