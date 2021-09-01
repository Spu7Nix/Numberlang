use core::panic;
use std::fmt::Debug;

use logos::Logos;

use crate::Number;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
enum Token {
    //OPERATORS
    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[token("<")]
    In,

    #[token(">")]
    Out,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[error]
    #[regex(r"[^\d\(\)]", logos::skip)]
    Error,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(Number),
    Tuple(Vec<Expression>),
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
}
#[derive(Debug)]
struct Tokens {
    tokens: Vec<(Token, String)>,
    index: usize,
}

impl Tokens {
    fn next(&mut self) -> Option<Token> {
        self.index += 1;
        Some(self.tokens.get(self.index)?.0)
    }

    fn previous(&mut self) -> Option<Token> {
        self.index -= 1;
        Some(self.tokens.get(self.index)?.0)
    }

    fn slice(&self) -> &str {
        &self.tokens[self.index].1
    }
}

// 1(69)

pub fn parse(unparsed: &str) -> Vec<Expression> {
    let mut raw_tokens = Token::lexer(unparsed);
    let mut tokens = Tokens {
        tokens: vec![(Token::Error, String::from(""))],
        index: 0,
    };

    while let Some(t) = raw_tokens.next() {
        tokens.tokens.push((t, raw_tokens.slice().to_string()));
    }

    let mut exprs = Vec::new();
    loop {
        match tokens.next() {
            Some(Token::Close) | None => {
                tokens.previous();
                break;
            }
            _ => {
                tokens.previous();
                exprs.push(parse_expression(&mut tokens));
            }
        }
    }

    exprs
}

fn parse_expression(tokens: &mut Tokens) -> Expression {
    let first = match tokens.next() {
        Some(Token::Number) => Expression::Number(tokens.slice().parse().unwrap()),
        Some(Token::Open) => {
            let mut exprs = Vec::new();
            loop {
                match tokens.next() {
                    Some(Token::Close) => {
                        break;
                    }
                    _ => {
                        tokens.previous();
                        exprs.push(parse_expression(tokens));
                    }
                }
            }

            if exprs.len() == 1 {
                exprs[0].clone()
            } else {
                Expression::Tuple(exprs)
            }
        }
        a => panic!("Expected expression, found {}", tokens.slice()),
    };
    if let Some(Token::In) = tokens.next() {
        let mut args = Vec::new();
        loop {
            match tokens.next() {
                Some(Token::Close) | None => {
                    tokens.previous();
                    break;
                }
                _ => {
                    tokens.previous();
                    args.push(parse_expression(tokens));
                }
            }
        }
        Expression::Call {
            func: Box::new(first),
            args,
        }
    } else {
        tokens.previous();
        first
    }
}

pub fn fmt(exprs: Vec<Expression>) -> String {
    if exprs.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut brackets_needed = false;
    let last_index = exprs.len() - 1;
    for (i, expr) in exprs.into_iter().enumerate() {
        match expr {
            Expression::Number(n) => {
                if brackets_needed {
                    out += &format!(" {}", n);
                } else {
                    out += &format!("{}", n);
                }
                brackets_needed = true;
            }
            Expression::Tuple(t) => {
                out += &format!("({})", fmt(t));
                brackets_needed = false;
            }
            Expression::Call { func, args } => {
                let brackets = i < last_index;
                if brackets {
                    out.push('(');
                } else if brackets_needed {
                    out.push(' ');
                }
                out += &fmt(vec![*func]);
                out.push('<');
                out += &fmt(args);

                if brackets {
                    out.push(')');
                }
                brackets_needed = false;
            }
        }
    }

    out
}
