use std::{any::TypeId, cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};
use lazy_static::lazy_static;
use chrono::{NaiveDate, NaiveDateTime};
use dynamic_queries_api::EventfulPeekable;

use crate::database::mssql::common::context::contextualizer::{ContextualizerMetadata, ContextualizerEntityDescription, ContextualizerColumnMetadata};
use crate::services::query::database::mssql::common::tokens::filter::token::{Token, Parentheses, Increment, Operation};

lazy_static! {
    static ref OPERATORS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("eq");
        set.insert("ne");
        set.insert("gt");
        set.insert("ge");
        set.insert("lt");
        set.insert("le");
        set.insert("in");
        set.insert("like");
        set
    };

    static ref VALID_OPERATORS: HashMap<TypeId, Vec<&'static str>> = {
        let mut map = HashMap::new();

        map.insert(TypeId::of::<String>(), vec!["eq", "ne", "like", "in"]);
        map.insert(TypeId::of::<Option<String>>(), vec!["eq", "ne", "like", "in"]);
        
        map.insert(TypeId::of::<bool>(), vec!["eq", "ne", "in"]);
        map.insert(TypeId::of::<Option<bool>>(), vec!["eq", "ne", "in"]);

        map.insert(TypeId::of::<i16>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<i32>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<i64>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<i128>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<i16>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<i32>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<i64>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<i128>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);

        map.insert(TypeId::of::<f32>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<f64>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<f32>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<f64>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);

        map.insert(TypeId::of::<NaiveDate>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<NaiveDateTime>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<NaiveDate>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        map.insert(TypeId::of::<Option<NaiveDateTime>>(), vec!["eq", "ne", "gt", "ge", "lt", "le", "in"]);
        
        map
    };
}

pub struct Tokenization;

impl Tokenization {
    fn format_value(value: &str, column_metadata: &ContextualizerColumnMetadata) -> Result<String, String> {
        let is_nullable = column_metadata.column_type() == TypeId::of::<Option<i32>>()
            || column_metadata.column_type() == TypeId::of::<Option<i64>>()
            || column_metadata.column_type() == TypeId::of::<Option<f32>>()
            || column_metadata.column_type() == TypeId::of::<Option<f64>>()
            || column_metadata.column_type() == TypeId::of::<Option<bool>>()
            || column_metadata.column_type() == TypeId::of::<Option<String>>()
            || column_metadata.column_type() == TypeId::of::<Option<NaiveDate>>()
            || column_metadata.column_type() == TypeId::of::<Option<NaiveDateTime>>();
    
        match value {
            "None" if is_nullable => Ok(String::from("NULL")),
            "None" => Err(format!(
                "None value not allowed for non-nullable column: {}",
                column_metadata.column_name()
            )),
            _ => match column_metadata.column_type() {

                _ if column_metadata.column_type() == TypeId::of::<Option<i32>>() ||
                     column_metadata.column_type() == TypeId::of::<Option<i64>>() ||
                     column_metadata.column_type() == TypeId::of::<Option<f32>>() ||
                     column_metadata.column_type() == TypeId::of::<Option<f64>>() => {
                        value.parse::<f64>().map_or_else(
                        |_| Err(format!("Invalid numeric value: {}", value)),
                        |_| Ok(value.to_string())
                    )
                }
    
                _ if column_metadata.column_type() == TypeId::of::<Option<bool>>() => {
                    match value.to_lowercase().as_str() {
                        "true" => Ok(String::from("TRUE")),
                        "false" => Ok(String::from("FALSE")),
                        _ => Err(format!("Invalid boolean value: {}", value)),
                    }
                }
    
                _ if column_metadata.column_type() == TypeId::of::<Option<String>>() ||
                     column_metadata.column_type() == TypeId::of::<Option<NaiveDate>>() ||
                     column_metadata.column_type() == TypeId::of::<Option<NaiveDateTime>>() => {
                    if value.starts_with("'") && value.ends_with("'") {
                        Ok(value.to_string())
                    } else {
                        Ok(format!("'{}'", value))
                    }
                }

                _ if column_metadata.column_type() == TypeId::of::<i32>() ||
                     column_metadata.column_type() == TypeId::of::<i64>() ||
                     column_metadata.column_type() == TypeId::of::<f32>() ||
                     column_metadata.column_type() == TypeId::of::<f64>() => {

                        value.parse::<f64>().map_or_else(
                        |_| Err(format!("Invalid numeric value: {}", value)),
                        |_| Ok(value.to_string())
                    )
                }
    
                _ if column_metadata.column_type() == TypeId::of::<bool>() => {
                    match value.to_lowercase().as_str() {
                        "true" => Ok(String::from("TRUE")),
                        "false" => Ok(String::from("FALSE")),
                        _ => Err(format!("Invalid boolean value: {}", value)),
                    }
                }
    
                _ if column_metadata.column_type() == TypeId::of::<String>() ||
                     column_metadata.column_type() == TypeId::of::<NaiveDate>() ||
                     column_metadata.column_type() == TypeId::of::<NaiveDateTime>() => {
                    if value.starts_with("'") && value.ends_with("'") {
                        Ok(value.to_string())
                    } else {
                        Ok(format!("'{}'", value))
                    }
                }
    
                _ => Err(format!("Unsupported column type for {}", column_metadata.column_name())),
            },
        }
    }

    fn split(text: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
    
        let mut parentheses_depth = 0;
        let mut in_single_quotes = false;
    
        for char in text.chars() {
            match char {
                '(' => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(char.to_string());
                    parentheses_depth += 1;
                }
                ')' => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(char.to_string());
                    parentheses_depth -= 1;
                }
                '\'' => {
                    in_single_quotes = !in_single_quotes;
                    current_token.push(char);
                }
                ' ' => {
                    if in_single_quotes {
                        current_token.push(char);
                    } else {
                        if !current_token.trim().is_empty() {
                            tokens.push(current_token.trim().to_string());
                            current_token.clear();
                        }
                    }
                }
                _ => {
                    current_token.push(char);
                }
            }
        }
    
        // [Add the last token if necessary]
        if !current_token.trim().is_empty() {
            tokens.push(current_token.trim().to_string());
        }
    
        // [Validate that parentheses are balanced]
        if parentheses_depth != 0 {
            return Err("Unbalanced parentheses".to_string());
        }

        // [Validate that single quote are matched]
        if in_single_quotes {
            return Err("Unmatched single quote".to_string());
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

    fn get_operation(column_metadata: &ContextualizerColumnMetadata, tokens_iter: &mut EventfulPeekable<std::slice::Iter<'_, String>>) -> Result<Operation, String> {

        let operator: &str = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {}", column_metadata.column_name()))?; 

        Self::validate_operator(operator, column_metadata)?;

        // [InValues]
        if operator == "in" {
            let mut non_formatted_values = String::from("");
            let mut parentheses_depth = 0;

            while let Some(in_token) = tokens_iter.next() {
                match in_token.as_str() {
                    "(" => { parentheses_depth += 1; },
                    ")" => { parentheses_depth -= 1; break; },
                    _ => {
                        non_formatted_values.push_str(in_token);
                    }
                }
            }
            
            if parentheses_depth != 0 {
                return Err(format!("Unbalanced parentheses in 'in clause': {} : {}", column_metadata.column_name(), operator));
            }

            // [Formatting  and Validation of values]
            let mut values = Vec::new();
            for non_formated_value in non_formatted_values.split(',') {
                match Self::format_value(non_formated_value, column_metadata) {
                    Ok(value) => values.push(value),
                    Err(err) => return Err(err)
                }
            }

            if values.len() == 0 {
                return Err(format!("'in clause' must have values: {} : {}", column_metadata.column_name(), operator));
            }

            return Ok(Operation::in_values(values))
        }

        // [Formatting and Validation of values]
        let non_formatted_value = tokens_iter.next().ok_or_else(|| format!("Invalid format near token: {} : {}", column_metadata.column_name(), operator))?;

        let value: String = Self::format_value(non_formatted_value, column_metadata)?;

        let operation = match operator {
            "eq" => Operation::equal(value),
            "ne" => Operation::not_equal(value),
            "gt" => Operation::greater_than(value),
            "ge" => Operation::greater_than_or_equal(value),
            "lt" => Operation::less_than(value),
            "le" => Operation::less_than_or_equal(value),
            "like" => Operation::like(value),
            _ => return Err(format!("Unsupported operator: {} : {}", column_metadata.column_name(), operator)),
        };
        
        Ok(operation)
    }

    fn validate_operator(operator: &str, column_metadata: &ContextualizerColumnMetadata) -> Result<(), String> {
        if !OPERATORS.contains(operator) {
            return Err(format!("Operator '{}' is not a valid operator", operator));
        }

        if let Some(valid_operators) = VALID_OPERATORS.get(&column_metadata.column_type()) {
            if valid_operators.contains(&operator) {
                return Ok(());
            } else {
                return Err(format!("Operator '{}' is not applicable to the column type", operator));
            }
        }

        Err(format!("Unsupported column type for operator '{}'", operator))
    }

    pub fn tokenize(text: &str, contextualizer: &ContextualizerMetadata) -> Result<Vec<Token>, String> {
   
        let mut tokens: Vec<Token> = Vec::new();
    
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
            Clause,
            Increment,
            ParenthesesBegin,
            ParenthesesEnd,
        }
    
        let mut last_token = LastToken::None;

        let error_with_context = |message: String| -> String {
            format!("{} [Context: {}]", message, context.borrow())
        };
    
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "(" => {

                    // [Ensure valid transitions to an opening parenthesis]
                    match last_token {
                        LastToken::Clause | LastToken::ParenthesesEnd => {
                            return Err(error_with_context(format!("Missing logical operator before '(' at token {}", token)));
                        }
                        _ => {
                            tokens.push(Token::new_parentheses(Parentheses::Begin));

                            last_token = LastToken::ParenthesesBegin;
                        }
                    }
                }
                ")" => {

                    // [Ensure valid transitions to a closing parenthesis]
                    match last_token {
                        LastToken::Increment | LastToken::ParenthesesBegin | LastToken::None => {       
                            return Err(error_with_context(format!("Unexpected ')' at token {}", token)));
                        }
                        _ => {
                            tokens.push(Token::new_parentheses(Parentheses::End));

                            last_token = LastToken::ParenthesesEnd;
                        }
                    }
                }
                "and" | "or" => {

                    // [Ensure logical operators are only between clauses or parentheses]
                    match last_token {
                        LastToken::Clause | LastToken::ParenthesesEnd => {
                            tokens.push(Token::new_increment(
                                if token == "and" {
                                    Increment::And
                                } else {
                                    Increment::Or
                                },
                            ));

                            last_token = LastToken::Increment;
                        }
                        _ => {
                            return Err(error_with_context(format!(
                                "Logical operator {} must follow a clause or ')'", token)
                            ));
                        }
                    }
                }
                _ => {

                    // [Handle clauses]
                    match last_token {
                        LastToken::Increment | LastToken::None | LastToken::ParenthesesBegin => {
                            
                            // [Current Metadata]
                            let (metadata, column_metadata) = Self::get_metadata(&metadata, token)
                            .map_err(|err| error_with_context(err))?;

                            // [Current Operation]
                            let operation = Self::get_operation(&column_metadata, &mut tokens_iter)
                            .map_err(|err| error_with_context(err))?;

                            tokens.push(Token::new_property(metadata, column_metadata,  operation));

                            last_token = LastToken::Clause;
                        }
                        _ => {

                            return Err(error_with_context(format!(
                                "Unexpected clause '{}' without a logical operator", token)
                            ));
                        }
                    }
                }
            }
        }
    
        match last_token {
            LastToken::Increment | LastToken::ParenthesesBegin => {
                return Err(error_with_context(String::from("Ends unexpectedly; incomplete structure.")));
            }
            _ => Ok(tokens),
        }
    }
}