/*
Given an arithmetic expression in Reverse Polish Notation, write a program to evaluate it.

The expression is given as a list of numbers and operands.
For example: [5, 3, '+'] should return 5 + 3 = 8.

For example, [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'] should return 5,
since it is equivalent to ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5.

You can assume the given expression is always valid.
*/

#![allow(dead_code)]

pub enum Term {
    Numeric(i32),
    Operator(char),
}

pub struct Expr {
    terms: Vec<Term>,
}

impl Expr {
    pub fn new(terms: Vec<Term>) -> Expr {
        Expr { terms }
    }
}

pub fn rpn(expr: &Expr) -> i32 {
    let mut stack = Vec::new();
    for term in &expr.terms {
        match term {
            Term::Numeric(n) => stack.push(*n),
            Term::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match op {
                    '+' => stack.push(a + b),
                    '-' => stack.push(a - b),
                    '*' => stack.push(a * b),
                    '/' => stack.push(a / b),
                    _ => (),
                }
            }
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn() {
        let expr = Expr::new(vec![
            Term::Numeric(15),
            Term::Numeric(7),
            Term::Numeric(1),
            Term::Numeric(1),
            Term::Operator('+'),
            Term::Operator('-'),
            Term::Operator('/'),
            Term::Numeric(3),
            Term::Operator('*'),
            Term::Numeric(2),
            Term::Numeric(1),
            Term::Numeric(1),
            Term::Operator('+'),
            Term::Operator('+'),
            Term::Operator('-'),
        ]);
        let result = rpn(&expr);
        assert_eq!(result, 5);
    }
}
