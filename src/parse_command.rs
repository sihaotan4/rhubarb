use nom::{
    bytes::complete::tag, character::complete::{alpha1, multispace1}, sequence::delimited, Finish, IResult
};
use std::collections::HashSet;
use crate::{database::Database, parse_set::{self, SetExpr}};

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
    Grant(String)
}

impl Database {
    pub fn resolve_command(
        &self,
        input: &str,
    ) -> anyhow::Result<CommandParseResult> {
        let command_raw = input.to_string();

        let (leftover, (database_operation, asset_set_expr, user_set_expr)) = match parse_command_to_expr(input).finish() {
            Ok(x) => Ok(x),
            Err(err) => Err(err.to_string()),
        }
        .unwrap();

        // check leftover - means parsing failed in some unexpected way
        if !leftover.is_empty() {
            return Err(anyhow::anyhow!(
                "Unexpected input was not parsed correctly: {}",
                leftover
            ));
        }

        // check permission validity 
        match database_operation.clone() {
            DatabaseOperationType::Grant(permission) => {
                if !self.valid_permissions.contains(&permission) {
                    return Err(anyhow::anyhow!("Invalid permission type"));
                }
            }
        }

        // resolve sets
        let asset_set_affected = parse_set::resolve_set(asset_set_expr, &self.asset_registry.data)?;
        let user_set_affected = parse_set::resolve_set(user_set_expr, &self.user_registry.data)?;
        
        let result = CommandParseResult {
            command_raw,
            database_operation,
            asset_set_affected,
            user_set_affected,
            metadata: CommandParseResultMetadata::new(),
        };

        anyhow::Ok(result)
    }
}

pub fn parse_command_to_expr(
    input: &str,
) -> IResult<&str, (DatabaseOperationType, SetExpr, SetExpr)> {
    // GRANT READ ON
    let (input, database_operation) = parse_operation(input)?;

    // Split the remaining input on " TO "
    let parts: Vec<&str> = input.splitn(2, " TO ").collect();

    // first set - the asset set e.g. (schema:tax EXCEPT table:sensitive_audit)
    let (leftover_1, asset_set_expr) = parse_set::parse_expr(parts[0])?;

    // second set - the user set e.g. (department:tax AND (designation:partner OR designation:senior))
    let (leftover_2, user_set_expr) = parse_set::parse_expr(parts[1])?;

    // Return the first leftover that is not empty - otherwise it's impossible to combine and return a string ref
    // but this is going to screw up the result if both fail 
    
    // WIP
    
    let leftover = if !leftover_1.trim().is_empty() {
        leftover_1
    } else if !leftover_2.trim().is_empty() {
        leftover_2
    } else {
        ""
    };

    Ok((leftover, (database_operation, asset_set_expr, user_set_expr)))
}

// Only supports GRANT now
pub fn parse_operation(input: &str) -> IResult<&str, DatabaseOperationType> {
    let (input, _) = tag("GRANT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = alpha1(input)?;

    // throwaway ON
    let (input, _) = tag(" ON ")(input)?;

    Ok((input, DatabaseOperationType::Grant(operation.to_string())))
}