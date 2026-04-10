use crate::lexer::Token;
use crate::ast::{Expr, Stmt};

// ── PARSER ───────────────────────────────────────────────────────────
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: &Token) {
        let t = self.consume();
        if &t != expected {
            panic!("Expected {:?} but got {:?}", expected, t);
        }
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
    match self.peek().clone() {
        Token::Let   => self.parse_let(),
        Token::Print => self.parse_print(),
        Token::If    => self.parse_if(),
        Token::While => self.parse_while(),
        Token::Ident(name) => {
            self.consume();
            // assignment: x = expr;
            if self.peek() == &Token::Equals {
                self.consume(); // =
                let value = self.parse_expr();
                self.expect(&Token::Semicolon);
                // treat assignment as re-binding the variable
                Stmt::Let { name, value }
            } else {
                // just an expression starting with an identifier
                let mut expr = Expr::Var(name);
                // handle any binary op after it
                let op = match self.peek() {
                    Token::Plus  => Some("+".to_string()),
                    Token::Minus => Some("-".to_string()),
                    Token::Star  => Some("*".to_string()),
                    Token::Slash => Some("/".to_string()),
                    _ => None,
                };
                if let Some(op) = op {
                    self.consume();
                    let right = self.parse_expr();
                    expr = Expr::BinOp { left: Box::new(expr), op, right: Box::new(right) };
                }
                if self.peek() == &Token::Semicolon { self.consume(); }
                Stmt::Expr(expr)
            }
        }
        _ => {
            let e = self.parse_expr();
            if self.peek() == &Token::Semicolon { self.consume(); }
            Stmt::Expr(e)
        }
    }
}

    fn parse_let(&mut self) -> Stmt {
        self.consume(); // let
        let name = match self.consume() {
            Token::Ident(s) => s,
            t => panic!("Expected identifier, got {:?}", t),
        };
        // skip optional : type annotation
        if self.peek() == &Token::Colon {
            self.consume(); // :
            self.consume(); // type name
        }
        self.expect(&Token::Equals);
        let value = self.parse_expr();
        self.expect(&Token::Semicolon);
        Stmt::Let { name, value }
    }

    fn parse_print(&mut self) -> Stmt {
        self.consume(); // print
        self.expect(&Token::LParen);
        let expr = self.parse_expr();
        self.expect(&Token::RParen);
        self.expect(&Token::Semicolon);
        Stmt::Print(expr)
    }

    fn parse_if(&mut self) -> Stmt {
        self.consume(); // if
        let cond = self.parse_expr();
        self.expect(&Token::LBrace);
        let mut then_block = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
            then_block.push(self.parse_stmt());
        }
        self.expect(&Token::RBrace);
        let mut else_block = Vec::new();
        if self.peek() == &Token::Else {
            self.consume();
            self.expect(&Token::LBrace);
            while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
                else_block.push(self.parse_stmt());
            }
            self.expect(&Token::RBrace);
        }
        Stmt::If { cond, then_block, else_block }
    }

    fn parse_while(&mut self) -> Stmt {
        self.consume(); // while
        let cond = self.parse_expr();
        self.expect(&Token::LBrace);
        let mut block = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
            block.push(self.parse_stmt());
        }
        self.expect(&Token::RBrace);
        Stmt::While { cond, block }
    }

    fn parse_expr(&mut self) -> Expr {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_additive();
        loop {
            let op = match self.peek() {
                Token::EqEq    => "==",
                Token::NotEq   => "!=",
                Token::Less    => "<",
                Token::Greater => ">",
                _ => break,
            }.to_string();
            self.consume();
            let right = self.parse_additive();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();
        loop {
            let op = match self.peek() {
                Token::Plus  => "+",
                Token::Minus => "-",
                _ => break,
            }.to_string();
            self.consume();
            let right = self.parse_multiplicative();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_primary();
        loop {
            let op = match self.peek() {
                Token::Star  => "*",
                Token::Slash => "/",
                _ => break,
            }.to_string();
            self.consume();
            let right = self.parse_primary();
            left = Expr::BinOp { left: Box::new(left), op, right: Box::new(right) };
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Token::Number(n)    => Expr::Number(n),
            Token::True         => Expr::Bool(true),
            Token::False        => Expr::Bool(false),
            Token::StringLit(s) => Expr::Str(s),
            Token::Ident(name)  => {
                if self.peek() == &Token::LParen {
                    self.consume();
                    let mut args = Vec::new();
                    while self.peek() != &Token::RParen {
                        args.push(self.parse_expr());
                    }
                    self.consume(); // )
                    Expr::Call { name, args }
                } else {
                    Expr::Var(name)
                }
            }
            Token::LParen => {
                let e = self.parse_expr();
                self.expect(&Token::RParen);
                e
            }
            t => panic!("Unexpected token in expression: {:?}", t),
        }
    }
}