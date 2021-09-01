mod parser;
use std::{collections::HashMap, fs};

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

#[derive(Debug, Clone)]
enum Value {
    Number(Number),
    Tuple(Vec<Value>),
}

struct State {
    variables: HashMap<Number, Value>,
}

fn eval(expr: parser::Expression, state: &mut State) -> Value {
    use parser::Expression::*;
    match expr {
        Number(n) => Value::Number(n),
        Tuple(v) => Value::Tuple(v.iter().map(|e| eval(e.clone(), state)).collect()),
        Call { func, args } => {
            let func = match eval(*func, state) {
                Value::Number(n) => n,
                _ => panic!("expected function id to be a number"),
            };
            let args = args.iter().map(|e| eval(e.clone(), state)).collect();
            call_function(func, args, state)
        }
    }
}

fn call_function(id: Number, args: Vec<Value>, state: &mut State) -> Value {
    match id {
        0 => {
            // get variable value
            assert_eq!(args.len(), 1);
            let var_id = match &args[0] {
                Value::Number(n) => n,
                Value::Tuple(v) => panic!("tuples can not be variable ids"),
            };
            state
                .variables
                .get(var_id)
                .unwrap_or_else(|| panic!("Could not find variable with id {}", var_id))
                .clone()
        }

        1 => {
            // set variable value
            assert_eq!(args.len(), 2);
            state.variables.insert(
                match &args[0] {
                    Value::Number(n) => *n,
                    Value::Tuple(v) => panic!("tuples can not be variable ids"),
                },
                args[1].clone(),
            );

            Value::Tuple(Vec::new())
        }

        10 => {
            // print text
            //dbg!(&args);
            let mut out = String::new();

            for arg in args {
                match arg {
                    Value::Number(n) => out.push(n as u8 as char),
                    Value::Tuple(v) => panic!("cannot print tuple"),
                }
            }

            print!("{}", out);
            Value::Tuple(Vec::new())
        }

        2 => {
            // sum
            let mut sum = 0;
            for arg in args {
                match arg {
                    Value::Number(n) => sum += n,
                    Value::Tuple(v) => panic!("cannot sum tuples"),
                }
            }
            Value::Number(sum)
        }

        _ => unimplemented!(),
    }
}
