
// GRANT READ ON asset:test_table TO division:product AND designation:intern

// GRANT READ ON asset:test_table TO (division:product AND designation:senior) OR designation:partner

use std::collections::{HashMap, HashSet};
use nom::{bytes::complete::{self, take_until, take_while}, combinator::map, IResult};

#[derive(Debug, PartialEq)]
enum SetExpr {
    Set(String),
    Union(Box<SetExpr>, Box<SetExpr>),          // OR
    Intersection(Box<SetExpr>, Box<SetExpr>),   // AND
    Except(Box<SetExpr>, Box<SetExpr>),         // EXCEPT which is shorthand for A n B'
}

fn parse_set(input: &str) -> IResult<&str, SetExpr> {
    let alphanumeric_or_special = |c: char| c.is_alphanumeric() || c == ':' || c == '_';
    let parser = take_while(alphanumeric_or_special);
    // nom map applies the parser to input, then applies the closure to the result
    map(parser, |s: &str| SetExpr::Set(s.to_string()))(input)
}

fn parse_set(input: &str) -> IResult<&str, SetExpr> {
    
}

fn parse(input: &str) -> anyhow::Result<(&str, SetExpr)> {
    let result = parse_set(input)
        .map_err(|err| anyhow::anyhow!("Parse error: {}", err))?;
    Ok(result)
}

fn resolve_set(input: &str, set_map: &HashMap<String, HashSet<String>>) -> anyhow::Result<HashSet<String>> {
    let (_, parsed_expr) = parse(input)?;

    let result = match parsed_expr {
        SetExpr::Set(key) => {
            set_map.get(&key)
                .ok_or(anyhow::anyhow!("Role key not found: {}", key))
                .map(|set| set.clone())
        },
        // Add more match arms here for other variants of `SetExpr`
        _ => {
            unimplemented!()
        }
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

        let set: HashSet<String> = "1 2 3 4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        
        map.insert("division:product_design3".to_string(), set.clone());

        // act
        let result = resolve_set("division:product_design3", &map).unwrap();
        
        assert_eq!(result, set, "Result should be equal to the set for input 'division:product_design3'");
    }

    // A OR B
    #[test]
    fn test_resolve_set_union() {
        // arrange
        let mut map = HashMap::new();
    
        let set_a: HashSet<String> = "1 2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_union: HashSet<String> = "1 2 3 4 5 6".split_whitespace().map(|s| s.to_string()).collect();
    
        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
    
        // act
        let result = resolve_set("A OR B", &map).unwrap();
    
        // assert
        assert_eq!(result, set_union, "Result should be equal to the union of sets A and B");
    }

    // (A AND B)
    // this one also tests the optional parentheses
    #[test]
    fn test_resolve_set_intersection() {
        // arrange
        let mut map = HashMap::new();
    
        let set_a: HashSet<String> = "1 2 3 4 5".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_intersection: HashSet<String> = "4 5".split_whitespace().map(|s| s.to_string()).collect();
    
        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
    
        // act
        let result = resolve_set("(A AND B)", &map).unwrap();
    
        // assert
        assert_eq!(result, set_intersection, "Result should be equal to the intersection of sets A and B");
    }

    // A EXCEPT B
    #[test]
    fn test_resolve_set_complement() {
        // arrange
        let mut map = HashMap::new();
    
        let set_a: HashSet<String> = "1 2 3 4 5".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "4 5 6".split_whitespace().map(|s| s.to_string()).collect();
        let set_complement: HashSet<String> = "1 2 3".split_whitespace().map(|s| s.to_string()).collect();
    
        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
    
        // act
        let result = resolve_set("(A EXCEPT B)", &map).unwrap();
    
        // assert
        assert_eq!(result, set_complement, "Result should be equal to the complement of sets A and B");
    }

    // A OR (B AND C)
    #[test]
    fn test_resolve_set_simple_nested_1() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act
        let result = resolve_set("A OR (B AND C)", &map).unwrap();

        // assert
        assert_eq!(result, set_result, "Result should be equal to the evaluation of the expression 'A OR (B AND C)'");
    }

    // (B OR C) OR A
    #[test]
    fn test_resolve_set_simple_nested_2() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3 4".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act
        let result = resolve_set("(B OR C) OR A", &map).unwrap();

        // assert
        assert_eq!(result, set_result, "Result should be equal to the evaluation of the expression '(B OR C) OR A'");
    }

    // (  B   OR C  ) OR A
    fn test_resolve_set_redundant_whitespace() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3 4".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);

        // act
        let result = resolve_set("(  B   OR C  ) OR A", &map).unwrap();

        // assert
        assert_eq!(result, set_result, "Result should be equal to the evaluation of the expression '(B OR C) OR A'");
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
        assert!(resolve_set("A OR B AND C", &map).is_err(), "Should return an error for bad syntax 'A OR B AND C'");
        assert!(resolve_set("(A OR B AND C)", &map).is_err(), "Should return an error for bad syntax '(A OR B AND C)'");
        assert!(resolve_set("AOR B", &map).is_err(), "Should return an error for bad syntax 'AOR B'");
    }

    // A OR ((C AND B) OR D)
    #[test]
    fn test_resolve_set_deeply_nested() {
        // arrange
        let mut map = HashMap::new();

        let set_a: HashSet<String> = "1 2".split_whitespace().map(|s| s.to_string()).collect();
        let set_b: HashSet<String> = "2 3".split_whitespace().map(|s| s.to_string()).collect();
        let set_c: HashSet<String> = "3 4".split_whitespace().map(|s| s.to_string()).collect();
        let set_d: HashSet<String> = "4 5".split_whitespace().map(|s| s.to_string()).collect();
        let set_result: HashSet<String> = "1 2 3 4 5".split_whitespace().map(|s| s.to_string()).collect();

        map.insert("A".to_string(), set_a);
        map.insert("B".to_string(), set_b);
        map.insert("C".to_string(), set_c);
        map.insert("D".to_string(), set_d);

        // act
        let result = resolve_set("A OR ((C AND B) OR D)", &map).unwrap();

        // assert
        assert_eq!(result, set_result, "Result should be equal to the evaluation of the expression 'A OR ((C AND B) OR D)'");
    }



}
