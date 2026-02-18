use crate::vars::store::{VarStore, Value};

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(String),
    LParen,
    RParen,
}

fn precedence(op: &str) -> i32 {
    match op {
        "or" => 1,
        "and" => 2,
        "==" | ">" | "<" | ">=" | "<=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
        "not" => 6,
        _ => 0,
    }
}

fn is_right_associative(op: &str) -> bool {
    op == "not"
}

fn apply_op(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        "%" => a % b,
        "==" => if (a - b).abs() < f64::EPSILON { 1.0 } else { 0.0 },
        ">" => if a > b { 1.0 } else { 0.0 },
        "<" => if a < b { 1.0 } else { 0.0 },
        ">=" => if a >= b { 1.0 } else { 0.0 },
        "<=" => if a <= b { 1.0 } else { 0.0 },
        "and" => if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 },
        "or" => if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 },
        _ => 0.0,
    }
}

fn apply_unary(a: f64, op: &str) -> f64 {
    match op {
        "not" => if a == 0.0 { 1.0 } else { 0.0 },
        _ => a,
    }
}

fn get_var(vars: &VarStore, name: &str) -> f64 {
    match vars.get(name) {
        Some(Value::Int(v)) => *v as f64,
        Some(Value::Float(v)) => *v as f64,
        Some(Value::Bool(v)) => if *v { 1.0 } else { 0.0 },
        _ => 0.0,
    }
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    let push_buffer = |buf: &mut String, tokens: &mut Vec<Token>| {
        if buf.is_empty() { return; }

        if let Ok(num) = buf.parse::<f64>() {
            tokens.push(Token::Number(num));
        } else {
            tokens.push(Token::Variable(buf.clone()));
        }
        buf.clear();
    };

    let mut chars = expr.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' => push_buffer(&mut buffer, &mut tokens),

            '(' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(Token::LParen);
            }

            ')' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(Token::RParen);
            }

            '+' | '-' | '*' | '/' | '%' => {
                push_buffer(&mut buffer, &mut tokens);
                tokens.push(Token::Operator(c.to_string()));
            }

            '=' | '>' | '<' => {
                push_buffer(&mut buffer, &mut tokens);

                if let Some('=') = chars.peek() {
                    let next = chars.next().unwrap();
                    tokens.push(Token::Operator(format!("{}{}", c, next)));
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                }
            }

            _ => buffer.push(c),
        }
    }

    push_buffer(&mut buffer, &mut tokens);

    // Convert keywords
    for token in &mut tokens {
        if let Token::Variable(s) = token {
            if s == "and" || s == "or" || s == "not" {
                *token = Token::Operator(s.clone());
            }
        }
    }

    tokens
}

pub fn eval(expr: &str, vars: &VarStore) -> f64 {
    let tokens = tokenize(expr);

    let mut values: Vec<f64> = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => values.push(*n),

            Token::Variable(name) => {
                values.push(get_var(vars, name));
            }

            Token::LParen => ops.push("(".into()),

            Token::RParen => {
                while ops.last().map(|s| s.as_str()) != Some("(") {
                    process_op(&mut values, &mut ops);
                }
                ops.pop();
            }

            Token::Operator(op) => {
                while let Some(top) = ops.last() {
                    if top == "(" {
                        break;
                    }

                    let cond = if is_right_associative(op) {
                        precedence(top) > precedence(op)
                    } else {
                        precedence(top) >= precedence(op)
                    };

                    if cond {
                        process_op(&mut values, &mut ops);
                    } else {
                        break;
                    }
                }

                ops.push(op.clone());
            }
        }

        i += 1;
    }

    while !ops.is_empty() {
        process_op(&mut values, &mut ops);
    }

    values.pop().unwrap_or(0.0)
}

fn process_op(values: &mut Vec<f64>, ops: &mut Vec<String>) {
    let op = ops.pop().unwrap();

    if op == "not" {
        let a = values.pop().unwrap();
        values.push(apply_unary(a, &op));
        return;
    }

    let b = values.pop().unwrap();
    let a = values.pop().unwrap();
    values.push(apply_op(a, b, &op));
}
