mod parser;
use std::{collections::HashMap, env::args, fs};

pub type Number = i32;
fn main() {
    let args: Vec<String> = args().collect();
    let content = fs::read_to_string(&args[1]).unwrap();
    let exprs = parser::parse(&content);

    println!("{}", parser::fmt(exprs.clone()));

    let mut state = State::default();

    for e in exprs {
        eval(&e, &mut state);
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

#[derive(Default, Clone)]
struct State {
    variables: HashMap<Number, Value>,
    functions: HashMap<Number, (Vec<Number>, Expression)>,
}
use parser::Expression;

fn eval(expr: &parser::Expression, state: &mut State) -> Value {
    use Expression::*;
    match expr {
        Number(n) => Value::Number(*n),
        Tuple(v) => Value::Tuple(v.iter().map(|e| eval(e, state)).collect()),
        Call { func, args } => {
            let func = eval(&**func, state).unwrap_num();
            call_function(func, args.clone(), state)
        }
    }
}

fn call_function(id: Number, args: Vec<Expression>, state: &mut State) -> Value {
    match id {
        0 => {
            // get variable value
            assert_eq!(args.len(), 1);
            let var_id = eval(&args[0], state).unwrap_num();
            state
                .variables
                .get(&var_id)
                .unwrap_or_else(|| panic!("Could not find variable with id {}", var_id))
                .clone()
        }

        1 => {
            // set variable value
            assert_eq!(args.len(), 2);
            let key = eval(&args[0], state).unwrap_num();

            let val = eval(&args[1], state);
            state.variables.insert(key, val);

            Value::Tuple(Vec::new())
        }

        2 => {
            // sum
            let mut sum = 0;
            for arg in args {
                sum += eval(&arg, state).unwrap_num();
            }
            Value::Number(sum)
        }

        3 => {
            // equal
            assert_eq!(args.len(), 2);

            Value::Number(if eval(&args[0], state) == eval(&args[1], state) {
                1
            } else {
                0
            })
        }

        4 => {
            // compare (returns 0 if less, 1 if equal, and 2 if more)
            assert_eq!(args.len(), 2);

            Value::Number(
                match eval(&args[0], state)
                    .unwrap_num()
                    .cmp(&eval(&args[1], state).unwrap_num())
                {
                    std::cmp::Ordering::Less => 0,
                    std::cmp::Ordering::Equal => 1,
                    std::cmp::Ordering::Greater => 2,
                },
            )
        }

        5 => {
            // mult
            let mut product = 1;
            for arg in args {
                product *= eval(&arg, state).unwrap_num();
            }
            Value::Number(product)
        }

        6 => {
            // subtract

            assert_eq!(args.len(), 2);

            Value::Number(eval(&args[0], state).unwrap_num() - eval(&args[1], state).unwrap_num())
        }

        7 => {
            // divide

            assert_eq!(args.len(), 2);

            Value::Number(eval(&args[0], state).unwrap_num() / eval(&args[1], state).unwrap_num())
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
                print_val(eval(&arg, state), &mut out);
            }

            print!("{}", out);
            Value::Tuple(Vec::new())
        }

        11 => {
            // convert to text
            assert_eq!(args.len(), 1);

            Value::Tuple(
                display(eval(&args[0], state))
                    .chars()
                    .map(|c| Value::Number(c as Number))
                    .collect(),
            )
        }

        20 => {
            // if
            assert_eq!(args.len(), 3);
            let condition = eval(&args[0], state);

            let is_true = !match condition {
                Value::Number(n) => n == 0,
                Value::Tuple(t) => t.is_empty(),
            };

            if is_true {
                eval(&args[1], state)
            } else {
                eval(&args[2], state)
            }
        }

        21 => {
            // while loop
            assert_eq!(args.len(), 2);
            loop {
                let condition = eval(&args[0], state);

                let is_true = !match condition {
                    Value::Number(n) => n == 0,
                    Value::Tuple(t) => t.is_empty(),
                };

                if is_true {
                    eval(&args[1], state);
                } else {
                    break;
                }
            }
            Value::Tuple(Vec::new())
        }

        22 => {
            // for loop
            assert_eq!(args.len(), 3);

            let iterator = eval(&args[0], state).unwrap_num();

            let saved_val = state.variables.get(&iterator).cloned();

            let list = match eval(&args[1], state) {
                n @ Value::Number(_) => vec![n],
                Value::Tuple(n) => n,
            };

            for val in list {
                state.variables.insert(iterator, val);
                eval(&args[2], state);
            }

            if let Some(v) = saved_val {
                state.variables.insert(iterator, v);
            }

            Value::Tuple(Vec::new())
        }

        30 => {
            // define function
            assert_eq!(args.len(), 3);
            let func_id = eval(&args[0], state).unwrap_num();
            if func_id < 100 {
                panic!("invalid function name (must be over 100)")
            }
            let arguments = match eval(&args[1], state) {
                Value::Number(n) => vec![n],
                Value::Tuple(t) => t.into_iter().map(|n| n.unwrap_num()).collect(),
            };

            state
                .functions
                .insert(func_id, (arguments, args[2].clone()));

            Value::Tuple(Vec::new())
        }

        40 => {
            // index tuple
            assert_eq!(args.len(), 2);
            let tuple = eval(&args[0], state);
            let index = eval(&args[1], state).unwrap_num();

            //dbg!(&tuple, index);

            assert!(index >= 0);

            match tuple {
                n @ Value::Number(_) => {
                    if index == 0 {
                        n
                    } else {
                        panic!("index out of bounds")
                    }
                }

                Value::Tuple(t) => t[index as usize].clone(),
            }
        }

        41 => {
            // tuple length
            assert_eq!(args.len(), 1);
            let tuple = eval(&args[0], state);
            match tuple {
                Value::Number(_) => Value::Number(1),

                Value::Tuple(t) => Value::Number(t.len() as Number),
            }
        }

        42 => {
            // create range
            assert_eq!(args.len(), 2);
            let start = eval(&args[0], state).unwrap_num();
            let end = eval(&args[1], state).unwrap_num();
            Value::Tuple((start..end).into_iter().map(Value::Number).collect())
        }

        43 => {
            // last element in tuple
            assert_eq!(args.len(), 1);
            let tuple = eval(&args[0], state);

            match tuple {
                n @ Value::Number(_) => n,

                Value::Tuple(t) => t.last().unwrap().clone(),
            }
        }

        44 => {
            // append to tuple
            assert_eq!(args.len(), 2);
            let tuple = eval(&args[0], state);
            let mut list = match tuple {
                n @ Value::Number(_) => vec![n],

                Value::Tuple(t) => t,
            };

            list.push(eval(&args[1], state));

            Value::Tuple(list)
        }

        n => {
            if let Some((argdef, body)) = state.functions.get(&n).cloned() {
                let prev_state = state.clone();

                assert_eq!(args.len(), argdef.len());

                for (i, arg) in argdef.iter().copied().enumerate() {
                    let val = eval(&args[i], state);
                    state.variables.insert(arg, val);
                }
                let out = eval(&body, state);
                *state = prev_state;
                out
            } else {
                panic!("undefined function {}", n)
            }
        }
    }
}

fn display(v: Value) -> String {
    match v {
        Value::Number(n) => n.to_string(),
        Value::Tuple(v) => {
            if v.is_empty() {
                return String::from("()");
            }
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
