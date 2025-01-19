use regex::Regex;
use std::{any::TypeId, cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};
use chrono::{NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;
use dynamic_queries_api::EventfulPeekable;

use crate::{database::mssql::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerEntityDescription, ContextualizerMetadata}, services::query::database::mssql::common::tokens::compute::token::{AliasOperation, ArithmeticOperation, Operand}};
use crate::services::query::database::mssql::common::tokens::compute::token::{Token, Operation};

lazy_static! {
    static ref OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("as");
        set
    }; 

    static ref ARITHMETIC_OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("add");
        set.insert("sub");
        set.insert("mul");
        set.insert("div");
        set.insert("mod");
        set
    }; 

    static ref VALID_ARITHMETIC_OPERATORS: HashMap<TypeId, Vec<&'static str>> = {
        let mut map = HashMap::new();

        map.insert(TypeId::of::<String>(), vec![]);
        map.insert(TypeId::of::<Option<String>>(), vec![]);
        
        map.insert(TypeId::of::<bool>(), vec![]);
        map.insert(TypeId::of::<Option<bool>>(), vec![]);

        map.insert(TypeId::of::<i16>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<i32>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<i64>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<i128>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<i16>>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<i32>>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<i64>>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<i128>>(), vec!["add", "sub", "mul", "div", "mod"]);

        map.insert(TypeId::of::<f32>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<f64>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<f32>>(), vec!["add", "sub", "mul", "div", "mod"]);
        map.insert(TypeId::of::<Option<f64>>(), vec!["add", "sub", "mul", "div", "mod"]);

        map.insert(TypeId::of::<NaiveDate>(), vec![]);
        map.insert(TypeId::of::<NaiveDateTime>(), vec![]);
        map.insert(TypeId::of::<Option<NaiveDate>>(), vec![]);
        map.insert(TypeId::of::<Option<NaiveDateTime>>(), vec![]);
        
        map
    };
}

pub struct Tokenization;

impl Tokenization {
    fn split(text: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        
        for char in text.chars() {
            match char {
                ',' => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(char.to_string());
                },
                ' ' => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                },
                _ => {
                    current_token.push(char);
                }
            }
        }
    
        // [Add the last token if necessary]
        if !current_token.trim().is_empty() {
            tokens.push(current_token.trim().to_string());
        }
    
        Ok(tokens)
    }

    fn resolve_nested_metadata(metadata: &ContextualizerEntityDescription, token: &str) -> Result<(ContextualizerEntityDescription, ContextualizerColumnMetadata), String> {
        let parts: Vec<&str> = token.split('/').collect();

        if parts.len() < 2 {
            return Err(format!("Invalid nested token format: {}", token));
        }

        let mut current_metadata = metadata.clone();

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                
                // [Last part should be a field in the related entity]
                let metadata = current_metadata.clone();

                let column_metadata = metadata
                    .columns
                    .get(*part)
                    .cloned()
                    .ok_or_else(|| format!("Invalid token in related entity: {}", part))?;

                return Ok((metadata, column_metadata));

            } else {
                // [Intermediate parts should be relationships]
                let relationship = current_metadata
                    .relationships
                    .get(*part)
                    .ok_or_else(|| format!("Invalid related entity: {}", part))?;

                // [Expanded]
                if !relationship.expanded {
                    return Err(format!("Non expanded relation: {}", part));
                }

                current_metadata = relationship.related_entity_metadata.clone();
            }
        }

        Err(format!("Unexpected error resolving metadata for token: {}", token))
    }

    fn resolve_non_nested_metadata(metadata: &ContextualizerEntityDescription, token: &str) -> Result<(ContextualizerEntityDescription, ContextualizerColumnMetadata), String> {
        
        let metadata = metadata.clone();

        let column_metadata = metadata
            .columns
            .get(token)
            .cloned()
            .ok_or_else(|| format!("Invalid token: {}", token))?;

            return Ok((metadata, column_metadata));
    }

    fn get_metadata(metadata: &ContextualizerEntityDescription, token: &str) -> Result<(ContextualizerEntityDescription, ContextualizerColumnMetadata), String> {
        if token.contains('/') {
            Self::resolve_nested_metadata(metadata, token)
        } else {
            Self::resolve_non_nested_metadata(metadata, token)
        }
    }
    
    fn get_operation(metadata: &ContextualizerEntityDescription, tokens_iter: &mut EventfulPeekable<std::slice::Iter<'_, String>>, token: &str) -> Result<Operation, String> {
        
        // [ArithmeticOperation]
        // [column_name + arithmetic_operator + column_name + alias + alias_name]

        // [AliasOperation]
        // [column_name + alias + alias_name]

        // [left-side metadata]
        let left_operand: &str = token; 

        let (left_operand_metadata, left_operand_column_metadata ) = Self::get_metadata(&metadata, left_operand)?;

        // [first-operator]
        let first_operator: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", token))?; 

        Self::validate_operator(first_operator)?;

        // [AliasOperation]
        if first_operator == "as" {
            
            // [alias]
            let alias_name: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", first_operator))?; 

            Self::validate_alias_name(alias_name)?;

            let alias_operation = 
                AliasOperation {
                    metadata: left_operand_metadata.clone(),
                    column_metadata: left_operand_column_metadata.clone(),
                    alias_name: alias_name.to_string(),
                    alias_type: left_operand_column_metadata.column_type(),
            };

            return Ok(Operation::Alias { operation: alias_operation });
        }

        // [ArithmeticOperation]

        // [right-side metadata]
        let right_operand: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", first_operator))?; 

        let (right_operand_metadata, right_operand_column_metadata ) = Self::get_metadata(&metadata, right_operand)?;

        Self::validate_arithmetic_operator(first_operator, &left_operand_column_metadata, &right_operand_column_metadata)?;

        // [second-operator]
        let second_operator: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", token))?; 

        if second_operator != "as" {
            return Err(format!("Operator '{}' is not a valid operator", second_operator));
        }

        let common_type = left_operand_column_metadata.column_type();

        // [alias]
        let alias_name: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", second_operator))?; 

        Self::validate_alias_name(alias_name)?;

        let arithmetic_operation = match first_operator {
            "add" => ArithmeticOperation::Addition {
                left_operand: Operand {
                    metadata: left_operand_metadata,
                    column_metadata: left_operand_column_metadata
                },
                right_operand: Operand {
                    metadata: right_operand_metadata,
                    column_metadata: right_operand_column_metadata
                },
                alias_name: alias_name.to_string(),
                alias_type: common_type
            },
            "sub" => ArithmeticOperation::Subtration {
                left_operand: Operand {
                    metadata: left_operand_metadata,
                    column_metadata: left_operand_column_metadata
                },
                right_operand: Operand {
                    metadata: right_operand_metadata,
                    column_metadata: right_operand_column_metadata
                },
                alias_name: alias_name.to_string(),
                alias_type: common_type
            },
            "mul" => ArithmeticOperation::Multiplication {
                left_operand: Operand {
                    metadata: left_operand_metadata,
                    column_metadata: left_operand_column_metadata
                },
                right_operand: Operand {
                    metadata: right_operand_metadata,
                    column_metadata: right_operand_column_metadata
                },
                alias_name: alias_name.to_string(),
                alias_type: common_type
            },
            "div" => ArithmeticOperation::Division {
                left_operand: Operand {
                    metadata: left_operand_metadata,
                    column_metadata: left_operand_column_metadata
                },
                right_operand: Operand {
                    metadata: right_operand_metadata,
                    column_metadata: right_operand_column_metadata
                },
                alias_name: alias_name.to_string(),
                alias_type: common_type
            },
            "mod" => ArithmeticOperation::Module {
                left_operand: Operand {
                    metadata: left_operand_metadata,
                    column_metadata: left_operand_column_metadata
                },
                right_operand: Operand {
                    metadata: right_operand_metadata,
                    column_metadata: right_operand_column_metadata
                },
                alias_name: alias_name.to_string(),
                alias_type: common_type
            },
            _ => return Err(format!("Unsupported arithmetic operator '{}' between  left-operand '{}' and right-operand '{}'", left_operand_column_metadata.column_name(), right_operand_column_metadata.column_name(), first_operator)),
        };

        return Ok(Operation::Arithmetic { operation:  arithmetic_operation });
    }

    fn validate_alias_name(alias_name: &str) -> Result<(), String> {
        
        let alias_name_regex = Regex::new(r"^[A-Za-z0-9-_]+$").unwrap();
        
        if !alias_name_regex.is_match(alias_name) {
            return Err(format!("Invalid alias name '{}'. [A-Za-z0-9-_]", alias_name));
        }

        Ok(())
    }

    fn validate_operator(operator: &str) -> Result<(), String> {
        if !OPERATORS.contains(operator) && !ARITHMETIC_OPERATORS.contains(operator) {
            return Err(format!("Operator '{}' is not a valid operator", operator));
        }
        Ok(())
    }

    fn validate_arithmetic_operator(operator: &str, left_operand_column_metadata: &ContextualizerColumnMetadata, right_operand_column_metadata: &ContextualizerColumnMetadata) -> Result<(), String> {
        
        if left_operand_column_metadata.column_type() != right_operand_column_metadata.column_type() {
            return Err(format!("The left-operand '{}' and right-operand '{}' must be same types to realizer the operation '{}'.", left_operand_column_metadata.column_name(), right_operand_column_metadata.column_name(), operator));
        }

        let common_type = left_operand_column_metadata.column_type();

        if let Some(valid_operators) = VALID_ARITHMETIC_OPERATORS.get(&common_type) {
            if !valid_operators.contains(&operator) {
                return Err(format!("The left-operand '{}' and right-operand '{}' cannot be apply to the operation '{}'", left_operand_column_metadata.column_name(), right_operand_column_metadata.column_name(), operator));
            } 
        }

        Ok(())
    }

    pub fn tokenize(text: &str, contextualizer: &ContextualizerMetadata) -> Result<Vec<Token>, String> {
    
        let mut tokens: Vec<Token> = Vec::new();

        let mut previous_alias: HashSet<String> = HashSet::new();
    
        let splited: &[String] = &Self::split(text)?;

        let metadata = contextualizer.get_context();

        // [Context]
        let context: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
            
        let mut tokens_iter = EventfulPeekable::new(
            splited.iter(),
            {
                let context = Rc::clone(&context);
                move |item| {
                    context.borrow_mut().push_str(&format!(" {} ", item));
                }
            },
        );

        // [Parser state]: Tracks the last token type
        enum LastToken {
            None,
            Comma,
            Clause
        }
    
        let mut last_token = LastToken::None;

        let error_with_context = |message: String| -> String {
            format!("{} [Context: {}]", message, context.borrow())
        };
    
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "," => {
                    match last_token {
                        LastToken::Clause => {
                            last_token = LastToken::Comma;
                        },
                        _ => {
                            return Err(error_with_context(String::from("Comma must follow a property")));
                        }
                    }
                },
                _ => {
                    match last_token {
                        LastToken::None | LastToken::Comma => {

                            // [Operation]
                            let operation: Operation = match Self::get_operation(&metadata, &mut tokens_iter, &token) {
                                Err(err) => return Err(error_with_context(err)),
                                Ok(value) => value
                            };
                
                            // [Alias]
                            let alias_name = match &operation {
                                Operation::Alias { operation } => &operation.alias_name,

                                Operation::Arithmetic { operation } => match operation {
                                    ArithmeticOperation::Addition { alias_name, .. } => alias_name,
                                    ArithmeticOperation::Subtration { alias_name, .. } => alias_name,
                                    ArithmeticOperation::Multiplication { alias_name, .. } => alias_name,
                                    ArithmeticOperation::Division { alias_name, .. } => alias_name,
                                    ArithmeticOperation::Module { alias_name, .. } => alias_name,
                                }
                            };

                            if metadata.columns.contains_key(alias_name) {
                                return Err(error_with_context(format!("The alias '{}' is reserved for an original table field with the same name.", alias_name)));
                            }
                        
                            if previous_alias.contains(alias_name) {
                                return Err(error_with_context(format!("The alias '{}' is already mapped.", alias_name)));
                            }
                            previous_alias.insert(alias_name.clone());

                            // [Token]
                            tokens.push(Token::new_operation(operation));

                            last_token = LastToken::Clause;
                        }
                        _ => {
                            return Err(error_with_context( String::from("Clause must follow a comma or be setted in begin")));
                        }
                    }
                }
            }
        }
        Ok(tokens)
    }
}