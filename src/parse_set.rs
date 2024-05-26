use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, multispace0},
    combinator::map,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub enum SetExpr {
    Set(String),
    Union(Box<SetExpr>, Box<SetExpr>),        // OR
    Intersection(Box<SetExpr>, Box<SetExpr>), // AND
    Except(Box<SetExpr>, Box<SetExpr>),       // EXCEPT which is shorthand for A n B'
}

// parses set lang only
// expecting this type of format (A OR (B AND C))
pub fn parse(
    input: &str,
    set_map: &HashMap<String, HashSet<String>>,
) -> anyhow::Result<HashSet<String>> {
    // need to transform the error to an owned error to prevent lifetime issues
    let (leftover, parsed_expr) = match parse_expr(input).finish() {
        Ok(x) => Ok(x),
        Err(err) => Err(err.to_string()),
    }
    .unwrap();

    // analyse leftover - means parsing failed in some unexpected way
    if !leftover.is_empty() {
        return Err(anyhow::anyhow!(
            "Unexpected input was not parsed correctly: {}",
            leftover
        ));
    }

    let result = resolve_set(parsed_expr, set_map)?;

    Ok(result)
}

fn parse_set(input: &str) -> IResult<&str, SetExpr> {
    let whitespace_or_parentheses = |c: char| c.is_whitespace() || c == '(' || c == ')';
    let parser = take_till(whitespace_or_parentheses);
    // nom map applies the parser to input, then applies the closure to the result
    map(parser, |s: &str| SetExpr::Set(s.to_string()))(input)
}

fn parse_union(input: &str) -> IResult<&str, SetExpr> {
    // parsing the inside of the parentheses
    let parser = tuple((parse_expr, tag(" OR "), parse_expr));

    // dealing with the parentheses
    let parser = delimited(char('('), parser, char(')'));

    map(parser, |(left, _, right)| {
        SetExpr::Union(Box::new(left), Box::new(right))
    })(input)
}

fn parse_intersection(input: &str) -> IResult<&str, SetExpr> {
    let parser = tuple((parse_expr, tag(" AND "), parse_expr));

    let parser = delimited(char('('), parser, char(')'));

    map(parser, |(left, _, right)| {
        SetExpr::Intersection(Box::new(left), Box::new(right))
    })(input)
}

fn parse_except(input: &str) -> IResult<&str, SetExpr> {
    let parser = tuple((parse_expr, tag(" EXCEPT "), parse_expr));

    let parser = delimited(char('('), parser, char(')'));

    map(parser, |(left, _, right)| {
        SetExpr::Except(Box::new(left), Box::new(right))
    })(input)
}

fn parse_parens(input: &str) -> IResult<&str, SetExpr> {
    delimited(
        preceded(multispace0, tag("(")),
        parse_expr,
        preceded(multispace0, tag(")")),
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, SetExpr> {
    alt((parse_parens, parse_set))(input)
}

pub fn parse_expr(input: &str) -> IResult<&str, SetExpr> {
    alt((parse_union, parse_intersection, parse_except, parse_term))(input)
}

pub fn resolve_set(
    parsed_expression: SetExpr,
    set_map: &HashMap<String, HashSet<String>>,
) -> anyhow::Result<HashSet<String>> {
    let result = match parsed_expression {
        SetExpr::Union(left, right) => {
            let left_set = resolve_set(*left, set_map)?;
            let right_set = resolve_set(*right, set_map)?;
            anyhow::Ok(
                left_set
                    .union(&right_set)
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>(),
            )
        }
        SetExpr::Intersection(left, right) => {
            let left_set = resolve_set(*left, set_map)?;
            let right_set = resolve_set(*right, set_map)?;
            anyhow::Ok(
                left_set
                    .intersection(&right_set)
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>(),
            )
        }
        SetExpr::Except(left, right) => {
            let left_set = resolve_set(*left, set_map)?;
            let right_set = resolve_set(*right, set_map)?;
            anyhow::Ok(
                left_set
                    .difference(&right_set)
                    .map(|s| s.to_string())
                    .collect::<HashSet<String>>(),
            )
        }
        SetExpr::Set(key) => set_map
            .get(&key)
            .ok_or(anyhow::anyhow!("Role not found: {}", key))
            .map(|set| set.clone()),
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_set_single_set() {
        // arrange
        let mut map = HashMap::new();

        let set: HashSet<String> = "1 2 3 4 5 6"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert("division:product_design3".to_string(), set.clone());

        // act
        let (_, parsed_expr) = parse_expr("division:product_design3").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        assert_eq!(
            result, set,
            "Result should be equal to the set for input 'division:product_design3'"
        );
    }

    // (A OR B)
    #[test]
    fn test_resolve_set_union() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_union: HashSet<String> = "1 2 3 4 5 6"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);

        // act
        let (_, parsed_expr) = parse_expr("(A OR B)").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_union,
            "Result should be equal to the union of sets A and B"
        );
    }

    // (A AND B)
    #[test]
    fn test_resolve_set_intersection() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2 3 4 5"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_intersection: HashSet<String> =
            "4 5".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);

        // act
        let (_, parsed_expr) = parse_expr("(A AND B)").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_intersection,
            "Result should be equal to the intersection of sets A and B"
        );
    }

    // A EXCEPT B
    #[test]
    fn test_resolve_set_complement() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2 3 4 5"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_complement: HashSet<String> =
            "1 2 3".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);

        // act
        let (_, parsed_expr) = parse_expr("(A EXCEPT B)").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_complement,
            "Result should be equal to the complement of set B intersected with A"
        );
    }

    // (A OR (B AND C))
    #[test]
    fn test_resolve_set_simple_nested_1() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> =
            "1 2 3".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act
        let (_, parsed_expr) = parse_expr("(A OR (B AND C))").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_result,
            "Result should be equal to the evaluation of the expression 'A OR (B AND C)'"
        );
    }

    // ((B OR C) OR A)
    #[test]
    fn test_resolve_set_simple_nested_2() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3 4"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act
        let (_, parsed_expr) = parse_expr("((B OR C) OR A)").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_result,
            "Result should be equal to the evaluation of the expression '(B OR C) OR A'"
        );
    }

    #[test]
    fn test_resolve_set_bad_syntax() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act and assert
        let (_, parsed_expr) = parse_expr("A OR B AND C").unwrap();
        assert!(
            resolve_set(parsed_expr, &map).is_err(),
            "Should return an error for bad syntax 'A OR B AND C'"
        );

        let (_, parsed_expr) = parse_expr("(A OR B AND C)").unwrap();
        assert!(
            resolve_set(parsed_expr, &map).is_err(),
            "Should return an error for bad syntax '(A OR B AND C)'"
        );

        let (_, parsed_expr) = parse_expr("AOR B").unwrap();
        assert!(
            resolve_set(parsed_expr, &map).is_err(),
            "Should return an error for bad syntax 'AOR B'"
        );
    }

    // (A OR ((C AND B) OR D))
    #[test]
    fn test_resolve_set_deeply_nested() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_d: HashSet<String> = "4 5".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3 4 5"
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);
        map.insert("D".to_string(), set_d);

        // act
        let (_, parsed_expr) = parse_expr("(A OR ((C AND B) OR D))").unwrap();
        let result = resolve_set(parsed_expr, &map).unwrap();

        // assert
        assert_eq!(
            result, set_result,
            "Result should be equal to the evaluation of the expression 'A OR ((C AND B) OR D)'"
        );
    }
}
