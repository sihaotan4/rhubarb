use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{char, multispace0},
    combinator::{cut, map},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use std::collections::{HashMap, HashSet};
use crate::{database::Database, parse_set};

#[derive(Debug, Clone)]
pub struct CommandParseResult {
    //command_origin: String,
    command_raw: String,
    database_operation: DatabaseOperationType,
    asset_set_affected: HashSet<String>, // set of asset ids
    user_set_affected: HashSet<String>,  // set of user ids
    metadata: CommandParseResultMetadata,
}

#[derive(Debug, Clone)]
pub struct CommandParseResultMetadata {
    //command_received_datetime: DateTime<Utc>,
    //asset_registry_etl_datetime: DateTime<Utc>,
    //user_registry_etl_datetime: DateTime<Utc>,
}
impl CommandParseResultMetadata {
    pub fn new() -> CommandParseResultMetadata {
        CommandParseResultMetadata {  }
    }
}

#[derive(Debug, Clone)]
pub enum DatabaseOperationType {
    GrantRead,
    GrantWrite,
    GrantOwnership,
}

// impl Database {
//     pub fn parse_command(
//         input: &str,
//         database: Database,
//     ) -> anyhow::Result<CommandParseResult> {
//         let command_raw = input.clone().to_string();

//         // GRANT READ ON
//         let (input, database_operation) = parse_operation(input)?;

//         // throwaway white space if any
//         let (input, _) = multispace0(input)?;

//         // first set - the asset set e.g. (schema:tax EXCEPT table:sensitive_audit)
//         let (input, asset_set_expr) = parse_set::parse_expr(input)?;
//         let asset_set_affected: HashSet<String> = parse_set::resolve_set(asset_set_expr, &database.asset_registry.data)?;
        
//         // throwaway TO
//         let (input, _) = delimited(multispace0, tag("TO"), multispace0)(input)?;
    
//         // second set - the user set e.g. (department:tax AND (designation:partner OR designation:senior))
//         let (leftover, user_set_expr) = parse_set::parse_expr(input)?;
//         let user_set_affected: HashSet<String> = parse_set::resolve_set(user_set_expr, &database.user_registry.data)?;

//         // analyse leftover - means parsing failed in some unexpected way
//         if !leftover.is_empty() {
//             return Err(anyhow::anyhow!(
//                 "Unexpected input was not parsed correctly: {}",
//                 leftover
//             ));
//         }

//         let result = CommandParseResult {
//             command_raw,
//             database_operation,
//             asset_set_affected,
//             user_set_affected,
//             metadata: CommandParseResultMetadata::new(),
//         };
    
//         Ok(result)
//     }
// }


pub fn parse_operation(input: &str) -> IResult<&str, DatabaseOperationType> {
    alt((
        map(tag("GRANT READ ON"), |_| DatabaseOperationType::GrantRead),
        map(tag("GRANT WRITE ON"), |_| DatabaseOperationType::GrantWrite),
        map(tag("GRANT OWNERSHIP ON"), |_| DatabaseOperationType::GrantOwnership),
    ))(input)
}

