use std::collections::HashMap;
use crate::ast::{Expr, Stmt};

// ── EVALUATOR ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if *n == n.floor() { write!(f, "{}", *n as i64) }
                else { write!(f, "{}", n) }
            }
            Value::Bool(b)   => write!(f, "{}", b),
            Value::Str(s)    => write!(f, "{}", s),
            Value::Nil       => write!(f, "nil"),
        }
    }
}

pub struct Interpreter {
    env: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { env: HashMap::new() }
    }

    pub fn run(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.exec(stmt);
        }
    }

    fn exec(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                let val = self.eval(value);
                self.env.insert(name.clone(), val);
            }
            Stmt::Print(expr) => {
                let val = self.eval(expr);
                println!("{}", val);
            }
            Stmt::If { cond, then_block, else_block } => {
                let cond_val = self.eval(cond);
                if self.is_truthy(&cond_val) {
                    self.run(then_block);
                } else {
                    self.run(else_block);
                }
            }
            Stmt::While { cond, block } => {
                loop {
                    let cond_val = self.eval(cond);
                    if !self.is_truthy(&cond_val) { break; }
                    self.run(block);
                }
            }
            Stmt::Expr(e) => { self.eval(e); }
        }
    }

    fn eval(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Number(n)  => Value::Number(*n),
            Expr::Bool(b)    => Value::Bool(*b),
            Expr::Str(s)     => Value::Str(s.clone()),
            Expr::Var(name)  => self.env.get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
            Expr::BinOp { left, op, right } => {
                let l = self.eval(left);
                let r = self.eval(right);
                self.apply_op(&l, op, &r)
            }
            Expr::Call { name, args } => {
                let vals: Vec<Value> = args.iter().map(|a| self.eval(a)).collect();
                match name.as_str() {
                    "print" => { for v in &vals { println!("{}", v); } Value::Nil }
                    _ => panic!("Unknown function: {}", name),
                }
            }
        }
    }

    fn apply_op(&self, l: &Value, op: &str, r: &Value) -> Value {
        match (l, r) {
            (Value::Number(a), Value::Number(b)) => match op {
                "+" => Value::Number(a + b),
                "-" => Value::Number(a - b),
                "*" => Value::Number(a * b),
                "/" => Value::Number(a / b),
                "==" => Value::Bool(a == b),
                "!=" => Value::Bool(a != b),
                "<"  => Value::Bool(a < b),
                ">"  => Value::Bool(a > b),
                _    => panic!("Unknown op: {}", op),
            },
            (Value::Str(a), Value::Str(b)) if op == "+" => Value::Str(a.clone() + b),
            (Value::Str(a), Value::Str(b)) if op == "==" => Value::Bool(a == b),
            (Value::Bool(a), Value::Bool(b)) if op == "==" => Value::Bool(a == b),
            _ => panic!("Type error: cannot apply '{}' to {:?} and {:?}", op, l, r),
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b)   => *b,
            Value::Number(n) => *n != 0.0,
            Value::Str(s)    => !s.is_empty(),
            Value::Nil       => false,
        }
    }
}