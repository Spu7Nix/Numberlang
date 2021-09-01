mod parser;
use std::{collections::HashMap, env::args, fs};

pub type Number = u32;
fn main() {
    let script_path = "test.123";
    let content = fs::read_to_string(script_path).unwrap();
    let exprs = parser::parse(&content);

    let mut state = State {
        variables: HashMap::new(),
    };

    for e in exprs {
        eval(e, &mut state);
    }
}

// Hello world: 0 < 72 101 108 108 111 32 119 111 114 108 100

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Number(Number),
    Tuple(Vec<Value>),
}

impl Value {
    fn unwrap_num(&self) -> Number {
        match self {
            Value::Number(n) => *n,
            _ => panic!("expected number"),
        }
    }
}

struct State {
    variables: HashMap<Number, Value>,
}
use parser::Expression;

fn eval(expr: parser::Expression, state: &mut State) -> Value {
    use Expression::*;
    match expr {
        Number(n) => Value::Number(n),
        Tuple(v) => Value::Tuple(v.iter().map(|e| eval(e.clone(), state)).collect()),
        Call { func, args } => {
            let func = eval(*func, state).unwrap_num();
            call_function(func, args, state)
        }
    }
}

fn call_function(id: Number, args: Vec<Expression>, state: &mut State) -> Value {
    match id {
        0 => {
            // get variable value
            assert_eq!(args.len(), 1);
            let var_id = eval(args[0].clone(), state).unwrap_num();
            state
                .variables
                .get(&var_id)
                .unwrap_or_else(|| panic!("Could not find variable with id {}", var_id))
                .clone()
        }

        1 => {
            // set variable value
            assert_eq!(args.len(), 2);
            let key = eval(args[0].clone(), state).unwrap_num();

            let val = eval(args[1].clone(), state);
            state.variables.insert(key, val);

            Value::Tuple(Vec::new())
        }

        2 => {
            // sum
            let mut sum = 0;
            for arg in args {
                sum += eval(arg, state).unwrap_num();
            }
            Value::Number(sum)
        }

        3 => {
            // equal
            assert_eq!(args.len(), 2);

            Value::Number(
                if eval(args[0].clone(), state) == eval(args[1].clone(), state) {
                    1
                } else {
                    0
                },
            )
        }

        4 => {
            // compare (returns 0 if less, 1 if equal, and 2 if more)
            assert_eq!(args.len(), 2);

            Value::Number(
                match eval(args[0].clone(), state)
                    .unwrap_num()
                    .cmp(&eval(args[1].clone(), state).unwrap_num())
                {
                    std::cmp::Ordering::Less => 0,
                    std::cmp::Ordering::Equal => 1,
                    std::cmp::Ordering::Greater => 2,
                },
            )
        }

        10 => {
            // print text
            fn print_val(v: Value, out: &mut String) {
                match v {
                    Value::Number(n) => out.push(n as u8 as char),
                    Value::Tuple(v) => {
                        for v in v {
                            print_val(v, out)
                        }
                    }
                }
            }

            let mut out = String::new();

            for arg in args {
                print_val(eval(arg, state), &mut out);
            }

            print!("{}", out);
            Value::Tuple(Vec::new())
        }

        11 => {
            // convert to text
            assert_eq!(args.len(), 1);

            Value::Tuple(
                display(eval(args[0].clone(), state))
                    .chars()
                    .map(|c| Value::Number(c as Number))
                    .collect(),
            )
        }

        20 => {
            // if
            assert_eq!(args.len(), 3);
            let condition = eval(args[0].clone(), state);

            let is_true = !match condition {
                Value::Number(n) => n == 0,
                Value::Tuple(t) => t.is_empty(),
            };

            if is_true {
                eval(args[1].clone(), state)
            } else {
                eval(args[2].clone(), state)
            }
        }

        21 => {
            // while loop
            assert_eq!(args.len(), 2);
            loop {
                let condition = eval(args[0].clone(), state);

                let is_true = !match condition {
                    Value::Number(n) => n == 0,
                    Value::Tuple(t) => t.is_empty(),
                };

                if is_true {
                    eval(args[1].clone(), state);
                } else {
                    break;
                }
            }
            Value::Tuple(Vec::new())
        }

        _ => unimplemented!(),
    }
}

fn display(v: Value) -> String {
    match v {
        Value::Number(n) => n.to_string(),
        Value::Tuple(v) => {
            let mut out = String::from("(");
            for v in v {
                out += &display(v);
                out.push(' ');
            }
            out.pop();
            out.push(')');
            out
        }
    }
}
