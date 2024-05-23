use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashSet;

#[derive(Debug)]
enum Expression {
    Set(HashSet<char>),
    Operation(Box<Expression>, char, Box<Expression>),
}

fn parse_set(input: &str) -> IResult<&str, Expression> {
    let (input, set) = delimited(
        tag("{"),
        take_while_m_n(1, 10, |c: char| c.is_alphabetic()),
        tag("}"),
    )(input)?;

    let set: HashSet<_> = set.chars().collect();
    Ok((input, Expression::Set(set)))
}

fn parse_operator(input: &str) -> IResult<&str, char> {
    let (input, _) = multispace0(input)?;
    let (input, op) = tag("∩")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, op.chars().next().unwrap()))
}

fn parse_parenthesized_expression(input: &str) -> IResult<&str, Expression> {
    delimited(
        tag("("),
        parse_expression,
        tag(")"),
    )(input)
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((parse_parenthesized_expression, parse_set))(input)
}

fn parse_set_operation(input: &str) -> IResult<&str, Expression> {
    let (input, (expr1, op, expr2)) = tuple((parse_expression, parse_operator, parse_expression))(input)?;

    Ok((
        input,
        Expression::Operation(Box::new(expr1), op, Box::new(expr2)),
    ))
}

fn execute_expression(expr: Expression) -> HashSet<char> {
    match expr {
        Expression::Set(set) => set,
        Expression::Operation(expr1, op, expr2) => {
            let set1 = execute_expression(*expr1);
            let set2 = execute_expression(*expr2);
            match op {
                '∩' => set1.intersection(&set2).cloned().collect(),
                _ => panic!("Unsupported operation"),
            }
        }
    }
}

fn main() {
    let (_, expr) = parse_set_operation("({a,b,c} ∩ {b,c,d}) ∩ {c,d,e}").unwrap();
    let result = execute_expression(expr);
    println!("{:?}", result);  // Outputs: {'c'}
}