use std::collections::VecDeque;
use crate::vars::store::{VarStore, Value};

#[derive(Debug, Clone)]
pub enum ExprToken {
    Number(f64),
    Variable(String),
    Operator(char),
    LParen,
    RParen,
}

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0,
    }
}

fn apply_op(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '%' => a % b,
        _ => 0.0,
    }
}

fn get_var(vars: &VarStore, name: &str) -> f64 {
    match vars.get(name) {
        Some(Value::Int(v)) => *v as f64,
        Some(Value::Float(v)) => *v as f64,
        _ => 0.0,
    }
}

pub fn tokenize(expr: &str) -> Vec<ExprToken> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let push_buffer = |buf: &mut String, tokens: &mut Vec<ExprToken>| {
        if buf.is_empty() {
            return;
        }

        if let Ok(num) = buf.parse::<f64>() {
            tokens.push(ExprToken::Number(num));
        } else {
            tokens.push(ExprToken::Variable(buf.clone()));
        }

        buf.clear();
    };

    for c in expr.chars() {
        match c {
            ' ' => {
                push_buffer(&mut buffer, &mut tokens);
            }
            '+' | '-' | '*' | '/' | '%' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(ExprToken::Operator(c));
            }
            '(' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(ExprToken::LParen);
            }
            ')' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(ExprToken::RParen);
            }
            _ => buffer.push(c),
        }
    }

    push_buffer(&mut buffer, &mut tokens);
    tokens
}

pub fn evaluate_expression(expr: &str, vars: &VarStore) -> f64 {
    let tokens = tokenize(expr);

    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    for token in tokens {
        match token {
            ExprToken::Number(n) => values.push(n),

            ExprToken::Variable(name) => {
                values.push(get_var(vars, &name));
            }

            ExprToken::LParen => ops.push('('),

            ExprToken::RParen => {
                while let Some(&op) = ops.last() {
                    if op == '(' {
                        ops.pop();
                        break;
                    }

                    let b = values.pop().unwrap();
                    let a = values.pop().unwrap();
                    let op = ops.pop().unwrap();
                    values.push(apply_op(a, b, op));
                }
            }

            ExprToken::Operator(op) => {
                while let Some(&top) = ops.last() {
                    if precedence(top) >= precedence(op) {
                        let b = values.pop().unwrap();
                        let a = values.pop().unwrap();
                        let op2 = ops.pop().unwrap();
                        values.push(apply_op(a, b, op2));
                    } else {
                        break;
                    }
                }
                ops.push(op);
            }
        }
    }

    while let Some(op) = ops.pop() {
        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        values.push(apply_op(a, b, op));
    }

    values.pop().unwrap_or(0.0)
}
