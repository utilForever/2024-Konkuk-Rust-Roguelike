// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    // An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op { op, left, right } => {
            let left = match eval(*left) {
                Ok(v) => v,
                Err(_msg) => return Err(String::from("left operand error")),
            };
            let right = match eval(*right) {
                Ok(v) => v,
                Err(_msg) => return Err(String::from("right operand error")),
            };

            match op {
                Operation::Add => add(left, right),
                Operation::Sub => sub(left, right),
                Operation::Mul => mul(left, right),
                Operation::Div => div(left, right),
            }
        }
        Expression::Value(v) => Ok(v),
    }
}

fn add(left: i64, right: i64) -> Result<i64, String> {
    if let Some(ret) = left.checked_add(right) {
        Ok(ret)
    } else {
        Err(String::from("Stack Overflow"))
    }
}

fn sub(left: i64, right: i64) -> Result<i64, String> {
    if let Some(ret) = left.checked_sub(right) {
        Ok(ret)
    } else {
        Err(String::from("Stack Overflow"))
    }
}

fn mul(left: i64, right: i64) -> Result<i64, String> {
    if let Some(ret) = left.checked_mul(right) {
        Ok(ret)
    } else {
        Err(String::from("Stack Overflow"))
    }
}

fn div(left: i64, right: i64) -> Result<i64, String> {
    if right == 0 {
        Err(String::from("Division by zero"))
    } else if let Some(ret) = left.checked_div(right) {
        Ok(ret)
    } else {
        Err(String::from("Stack Overflow"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            Ok(85)
        );
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("Division by zero"))
        );
    }

    #[test]
    fn test_stack_overflow() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(i64::MAX)),
                right: Box::new(Expression::Value(10)),
            }),
            Err(String::from("Stack Overflow"))
        );
    }
}
