use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::database::sqlite::common::context::contextualizer::{ContextualizerMetadata, ContextualizerColumnMetadata};
use crate::services::query::database::sqlite::common::tokens::filter::token::{Increment, Operation, Parentheses, Token};

use super::compute::compute::Compute;
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

pub struct Filter;

impl Filter {   
    pub fn process(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let error = |message: String| -> String {
            format!("Invalid $filter: {}", message)
        };

        let mut properties: Vec<String> = Vec::new();

        // [Handle the Internal Specification here]

        if !contextualizer.ignore_rules.filter {
            if let Some(text) = text {

                let tokens = match Tokenization::tokenize(text, contextualizer) {
                    Err(err) => return Err(error(err)),
                    Ok(value) => value
                };
    
                properties.extend(
                    match Self::process_with_tokens(&tokens, alias_manager) {
                        Err(err) => return Err(error(err)),
                        Ok(value) => value
                    });

                // [There no changes in context for while...]
            }
        }

        let mut sql = String::new();

        if !properties.is_empty() {

            sql = match Self::build_string_statement(&properties) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };
        }

        Ok(sql)
    }
    
    fn build_string_statement(properties: &Vec<String>) -> Result<String, String> {
        let mut sql: String = String::new();
    
        sql.push_str("WHERE\n");
    
        let mut needs_space = false;
    
        for property in properties {
            match property.as_str() {
                "(" => {
                    if needs_space {
                        sql.push_str(" ");
                    }
                    sql.push_str("(");
                    needs_space = false;
                }
                ")" => {
                    sql.push_str(")");
                    needs_space = true;
                }
                _ => {
                    if needs_space {
                        sql.push_str(" ");
                    }
                    sql.push_str(property);
                    needs_space = true;
                }
            }
        }
    
        Ok(sql)
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<Vec<String>, String> {
        
        let mut properties: Vec<String> = Vec::new();

        let get_operator = |operator_type: &OperationFilterType| -> &'static str {
            OPERATORS
                .get(operator_type)
                .ok_or_else(|| format!("Operator not found for {:?}", operator_type))
                .unwrap()
        };

        for token in tokens {
            match token {
                Token::Parentheses { parentheses } => {
                    match parentheses {
                        Parentheses::End => properties.push(String::from(")")),
                        Parentheses::Begin => properties.push(String::from("(")),             
                    }
                },
                Token::Increment { increment } => {
                    match increment {
                        Increment::And => properties.push(String::from("AND")),
                        Increment::Or => properties.push(String::from("OR"))
                    }
                },
                Token::Property { metadata, column_metadata, operation } => {
                    match column_metadata {
                        ContextualizerColumnMetadata::Original { column_name, .. } => {

                            let alias = alias_manager.get_or_create_table_alias(&metadata.table_name);

                                match &operation {
                                    Operation::Equal { value } => {

                                        let operator = get_operator(&OperationFilterType::Equal);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {

                                        let operator = get_operator(&OperationFilterType::NotEqual);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {

                                        let operator = get_operator(&OperationFilterType::GreaterThan);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {

                                        let operator = get_operator(&OperationFilterType::GreaterThanOrEqual);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {

                                        let operator = get_operator(&OperationFilterType::LessThan);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                        
                                        let operator = get_operator(&OperationFilterType::LessThanOrEqual);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                        
                                        let operator = get_operator(&OperationFilterType::Like);

                                        properties.push(format!(
                                            "[{}].[{}] {} {}",
                                            alias, column_name, operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {

                                        let operator = get_operator(&OperationFilterType::InValues);

                                        properties.push(format!(
                                            "[{}].[{}] {} ({})",
                                            alias, column_name, operator, value.join(",")
                                        ));
                                    },
                                }
                        },
                        ContextualizerColumnMetadata::Dynamic { .. } => { 
                            Compute::process_computed_column(column_metadata, operation, &mut properties, alias_manager)?
                        } 
                    }  
                },
            }
        }
        Ok(properties)
    }
}
