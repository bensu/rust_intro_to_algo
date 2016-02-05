// Evaluate Expressions

mod stack;

use stack::Stack;

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

use Op::*;

fn to_op(c: char) -> Option<Op> {
    match c {
        '+' => Some(Plus),
        '-' => Some(Minus),
        '*' => Some(Multiply),
        '/' => Some(Divide),
        _ => None,
    }
}

fn apply_op(op: Op, a: u32, b: u32) -> u32 {
    match op {
        Plus => a + b,
        Minus => a - b,
        Multiply => a * b,
        Divide => a / b,
    }
}

fn eval(expression: &str) -> u32 {
    let mut op_stack = stack::List::<Op>::new();
    let mut num_stack = stack::List::<u32>::new();
    for c in expression.chars() {
        if c.is_numeric() {
            num_stack.push((c as u32) - 48);
        } else if (c == '(') || c.is_whitespace() {
            ();
        } else if c == ')' {
            let a = match num_stack.pop() {
                None => { panic!("Unbalanced parens"); 0 },
                Some(n) => n,
            };
            let b = match num_stack.pop() {
                None => { panic!("Unbalanced parens"); 0 },
                Some(n) => n,
            };
            let op = match op_stack.pop() {
                None => { panic!("Unbalanced parens"); Plus },
                Some(o) => o,
            };
            num_stack.push(apply_op(op, a, b));
        } else {
            // try to process as op
            let maybe_op = to_op(c);
            match maybe_op {
                None => panic!("Character not allowed: {}", c),
                Some(op) => op_stack.push(op),
            }
        }
    }
    match num_stack.pop() {
        None => { panic!("Unbalanced parens"); 0 },
        Some(result) => result,
    }
}

fn main() {
    let exp = "(2 + 1)";
    println!("{} = {:?}", exp, eval(exp));
}
