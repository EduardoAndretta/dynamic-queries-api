use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;
use crate::services::query::database::sqlite::common::tokens::filter::token::{Increment, Operation, Parentheses, Token};

use super::token::tokenization::Tokenization;

#[derive(Hash, Eq, PartialEq, Debug)]
enum OperationFilterType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    InValues,
}

lazy_static! {
    static ref OPERATORS: HashMap<OperationFilterType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(OperationFilterType::Equal, "=");
        map.insert(OperationFilterType::NotEqual, "<>");
        map.insert(OperationFilterType::GreaterThan, ">");
        map.insert(OperationFilterType::GreaterThanOrEqual, ">=");
        map.insert(OperationFilterType::LessThan, "<");
        map.insert(OperationFilterType::LessThanOrEqual, "<=");
        map.insert(OperationFilterType::Like, "LIKE");
        map.insert(OperationFilterType::InValues, "IN");
        map
    };
}

pub struct FilterSqlite;

impl FilterSqlite {   
    pub fn process<T: EntityMetadata>(
        text: &str,
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {

        let tokens = Tokenization::tokenize::<T>(text)?;

        Self::process_with_tokens(&tokens, alias_manager)
    }
    
    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();
           
        sql.push_str("WHERE\n");

        let get_operator = |operator_type: &OperationFilterType| -> &'static str {
            OPERATORS
                .get(operator_type)
                .ok_or_else(|| format!("Invalid $filter: Operator not found for {:?}", operator_type))
                .unwrap()
        };

        for token in tokens {

            sql.push_str(" ");

            match token {
                Token::Parentheses { parentheses } => {
                    match parentheses {
                        Parentheses::End => { 
                            sql.push_str(")");                 
                        },
                        Parentheses::Begin => { 
                            sql.push_str("(");                 
                  
                        }
                    }
                },
                Token::Increment { increment } => {
                    match increment {
                        Increment::And => sql.push_str("OR"),
                        Increment::Or => sql.push_str("AND")
                    }
                },
                Token::Property { metadata, column_metadata, operation } => {
                    let alias = alias_manager.get_or_create_table_alias(&metadata.table_name);

                    match &operation {
                        Operation::Equal { value } => {

                            let operator = get_operator(&OperationFilterType::Equal);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::NotEqual { value } => {

                            let operator = get_operator(&OperationFilterType::NotEqual);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::GreaterThan { value } => {

                            let operator = get_operator(&OperationFilterType::GreaterThan);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::GreaterThanOrEqual { value } => {

                            let operator = get_operator(&OperationFilterType::GreaterThanOrEqual);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::LessThan { value } => {

                            let operator = get_operator(&OperationFilterType::LessThan);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::LessThanOrEqual { value } => {
                            
                            let operator = get_operator(&OperationFilterType::LessThanOrEqual);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::Like { value } => {
                            
                            let operator = get_operator(&OperationFilterType::Like);

                            sql.push_str(&format!(
                                "[{}].[{}] {} {}",
                                alias, column_metadata.column_name, operator, value
                            ));
                        },
                        Operation::InValues { value } => {

                            let operator = get_operator(&OperationFilterType::InValues);

                            sql.push_str(&format!(
                                "[{}].[{}] {} ({})",
                                alias, column_metadata.column_name, operator, value.join(",")
                            ));
                        },
                    }
                },
            }
        }

        sql.push_str("\n");

        Ok(sql.clone())
    }
}
