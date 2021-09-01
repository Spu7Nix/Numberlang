mod parser;
use std::fs;

pub type Number = u32;
fn main() {
    let script_path = "test.123";
    let content = fs::read_to_string(script_path).unwrap();
    let exprs = parser::parse(&content);

    for e in exprs {
        eval(e);
    }
}

// Hello world: 0 < 72 101 108 108 111 32 119 111 114 108 100

#[derive(Debug)]
enum Value {
    Number(Number),
    Tuple(Vec<Value>),
}
struct State {}

fn eval(expr: parser::Expression) -> Value {
    use parser::Expression::*;
    match expr {
        Number(n) => Value::Number(n),
        Tuple(v) => Value::Tuple(v.iter().map(|e| eval(e.clone())).collect()),
        Call { func, args } => {
            let func = match eval(*func) {
                Value::Number(n) => n,
                _ => panic!("expected function id to be a number"),
            };
            let args = args.iter().map(|e| eval(e.clone())).collect();
            call_function(func, args)
        }
    }
}

fn call_function(id: Number, args: Vec<Value>) -> Value {
    match id {
        1 => {
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
