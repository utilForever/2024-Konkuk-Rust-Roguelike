use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    // A reference to a variable.
    Var(String),
    // A literal number.
    Number(u32),
    // A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

#[derive(Error, Debug)]
enum TokenizerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unexpected end of input/file")]
    UnexpectedEOF,
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.0.next()?;

        Some(match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Ok(Token::Number(num))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Ok(Token::Identifier(ident))
            }
            '+' => Ok(Token::Operator(Op::Add)),
            '-' => Ok(Token::Operator(Op::Sub)),
            _ => Err(TokenizerError::UnexpectedCharacter(c)),
        })
    }
}

fn parse(input: &str) -> Result<Expression, TokenizerError> {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Result<Expression, TokenizerError> {
        let Some(tok_or_err) = tokens.next() else {
            return Err(TokenizerError::UnexpectedEOF);
        };

        let Ok(tok) = tok_or_err else {
            return Err(tok_or_err.unwrap_err());
        };

        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("Invalid 32-bit integer'");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => {
                return Err(TokenizerError::UnexpectedToken(tok));
            }
        };

        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => Ok(expr),
            Some(Ok(Token::Operator(op))) => Ok(Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)?),
            )),
            Some(Err(token_error)) => Err(token_error),
            Some(Ok(tok)) => Err(TokenizerError::UnexpectedToken(tok)),
        }
    }

    parse_expr(&mut tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expression_valid() {
        let expr = parse("10+foo+20-30");
        assert!(expr.is_ok());
        assert_eq!(format!("{:?}", expr.unwrap()), "Operation(Number(10), Add, Operation(Var(\"foo\"), Add, Operation(Number(20), Sub, Number(30))))");
    }

    #[test]
    fn expression_invalid() {
        let expr = parse("10+foo+20-");
        assert!(expr.is_err());
        assert_eq!(format!("{:?}", expr.unwrap_err()), "UnexpectedEOF");
    }
}
