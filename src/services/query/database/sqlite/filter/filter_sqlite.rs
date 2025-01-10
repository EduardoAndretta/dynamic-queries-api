use std::any::TypeId;
use chrono::{NaiveDate, NaiveDateTime};

use crate::dto::metadata::ColumnMetadata;
use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;

pub struct FilterSqlite;

impl FilterSqlite {
    
    fn format_sql_value(value: Option<&str>, column_metadata: &ColumnMetadata) -> Result<String, String> {

        println!("{:#?}", column_metadata);

        let is_nullable = column_metadata.column_type == TypeId::of::<Option<i32>>()
            || column_metadata.column_type == TypeId::of::<Option<i64>>()
            || column_metadata.column_type == TypeId::of::<Option<f32>>()
            || column_metadata.column_type == TypeId::of::<Option<f64>>()
            || column_metadata.column_type == TypeId::of::<Option<bool>>()
            || column_metadata.column_type == TypeId::of::<Option<String>>()
            || column_metadata.column_type == TypeId::of::<Option<NaiveDate>>()
            || column_metadata.column_type == TypeId::of::<Option<NaiveDateTime>>();
    
        match value {
            None if is_nullable => Ok("NULL".to_string()),
            None => Err(format!(
                "NULL value not allowed for non-nullable column: {}",
                column_metadata.column_name
            )),
            Some(value) => match column_metadata.column_type {

                _ if column_metadata.column_type == TypeId::of::<Option<i32>>() ||
                     column_metadata.column_type == TypeId::of::<Option<i64>>() ||
                     column_metadata.column_type == TypeId::of::<Option<f32>>() ||
                     column_metadata.column_type == TypeId::of::<Option<f64>>() => {
                        value.parse::<f64>().map_or_else(
                        |_| Err(format!("Invalid numeric value: {}", value)),
                        |_| Ok(value.to_string())
                    )
                }
    
                _ if column_metadata.column_type == TypeId::of::<Option<bool>>() => {
                    match value.to_lowercase().as_str() {
                        "true" => Ok("TRUE".to_string()),
                        "false" => Ok("FALSE".to_string()),
                        _ => Err(format!("Invalid boolean value: {}", value)),
                    }
                }
    
                _ if column_metadata.column_type == TypeId::of::<Option<String>>() ||
                     column_metadata.column_type == TypeId::of::<Option<NaiveDate>>() ||
                     column_metadata.column_type == TypeId::of::<Option<NaiveDateTime>>() => {
                    if value.starts_with("'") && value.ends_with("'") {
                        Ok(value.to_string())
                    } else {
                        Ok(format!("'{}'", value))
                    }
                }

                _ if column_metadata.column_type == TypeId::of::<i32>() ||
                     column_metadata.column_type == TypeId::of::<i64>() ||
                     column_metadata.column_type == TypeId::of::<f32>() ||
                     column_metadata.column_type == TypeId::of::<f64>() => {

                        println!("{:#?}, {:#?}", value, column_metadata.column_name);


                        value.parse::<f64>().map_or_else(
                        |_| Err(format!("Invalid numeric value: {}", value)),
                        |_| Ok(value.to_string())
                    )
                }
    
                _ if column_metadata.column_type == TypeId::of::<bool>() => {
                    match value.to_lowercase().as_str() {
                        "true" => Ok("TRUE".to_string()),
                        "false" => Ok("FALSE".to_string()),
                        _ => Err(format!("Invalid boolean value: {}", value)),
                    }
                }
    
                _ if column_metadata.column_type == TypeId::of::<String>() ||
                     column_metadata.column_type == TypeId::of::<NaiveDate>() ||
                     column_metadata.column_type == TypeId::of::<NaiveDateTime>() => {
                    if value.starts_with("'") && value.ends_with("'") {
                        Ok(value.to_string())
                    } else {
                        Ok(format!("'{}'", value))
                    }
                }
    
                _ => Err(format!("Unsupported column type for {}", column_metadata.column_name)),
            },
        }
    }

    fn tokenize_filter(filter: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut parentheses_depth = 0; // Track parentheses depth to handle nested ones
        
        let mut in_single_quotes = false; // Track whether we're inside single quotes (for complex tokens with spaces)
        
        for char in filter.chars() {
            match char {
                '(' => {
                    // Push the current token if it's not empty, then clear it
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
                ' ' => {
                    if parentheses_depth == 0 && !in_single_quotes && !current_token.trim().is_empty() {

                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    } else {
                        current_token.push(char);
                    }
                }
                '\'' => {
                    in_single_quotes = !in_single_quotes;
                    current_token.push(char);
                }
                _ => {
                    current_token.push(char);
                }
            }
        }
    
        // Add the last token if necessary
        if !current_token.trim().is_empty() {
            tokens.push(current_token.trim().to_string());
        }
    
        Ok(tokens)
    }

    pub fn validate<T: EntityMetadata>(
        filter: &str,
    ) -> Result<(), String> {

        if filter.trim().is_empty() {
            return Err("Invalid $filter: cannot be empty".into());
        }
    
        // Tokenize the filter clause
        let tokens = Self::tokenize_filter(filter)?;
    
        // Validate each token based on the entity metadata
        Self::validate_filter_tokens::<T>(&tokens)?;
        
        Ok(())
    }

    fn validate_filter_tokens<T: EntityMetadata>(tokens: &[String]) -> Result<(), String> {
        let metadata = T::metadata();
        let mut tokens_iter = tokens.iter().peekable(); // Create an iterator that we can peek ahead with
        
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "(" => {

                    let mut nested_tokens = Vec::new();
                    let mut parentheses_depth = 1;
                    
                    while let Some(inner_token) = tokens_iter.next() {
                        if inner_token == "(" {
                            parentheses_depth += 1;
                        } else if inner_token == ")" {
                            parentheses_depth -= 1;
                            if parentheses_depth == 0 {
                                break;
                            }
                        }
                        nested_tokens.push(inner_token.clone());
                    }
    
                    if parentheses_depth != 0 {
                        return Err("Unbalanced parentheses in $filter".into());
                    }
    
                    Self::validate_filter_tokens::<T>(&nested_tokens)?;
                }
                "and" | "or" => {
                    if tokens_iter.peek().is_none() {
                        return Err(format!("Logical operator '{}' must be between clauses", token));
                    }
                }
                "in" => {
                    if let Some(field) = tokens_iter.next() {
                        let value = tokens_iter.next().ok_or_else(|| format!("Invalid $filter format near token: {}", field))?;
    
                        if field.contains("/") {
                            let parts: Vec<&str> = field.split('/').collect();
                            if parts.len() >= 2 {

                                let mut last_entity_metadata = metadata.clone();
                                for (i, part) in parts.iter().enumerate() {
                                    
                                    // For each part of the path, check if it's a valid relationship or column
                                    if i == parts.len() - 1 {
                                        // Last part should be a field in the related entity
                                        if !last_entity_metadata.columns.contains_key(*part) {
                                            return Err(format!("Invalid field in related entity: {} in filter", *part));
                                        }
                                    } else {
                                        // Intermediate parts should be relationships
                                        if !last_entity_metadata.relationships.contains_key(*part) {
                                            return Err(format!("Invalid related entity: {} in filter", *part));
                                        }
                                        last_entity_metadata = last_entity_metadata.relationships.get(*part).unwrap().related_entity_metadata.clone();
                                    }
                                }
                            } else {
                                return Err(format!("Invalid nested field format in filter: {}", field));
                            }
                        } else {
                            if !metadata.columns.contains_key(field) {
                                return Err(format!("Invalid field in filter: {}", field));
                            }
                        }
    
                        let column_metadata = metadata.columns.get(field)
                            .ok_or_else(|| format!("Field '{}' not found in metadata columns", field))?;
    
                        if value.starts_with("(") && value.ends_with(")") {
                            let values = value[1..value.len() - 1]
                                .split(',')
                                .map(|v| v.trim().to_string())
                                .collect::<Vec<String>>();
    
                            for v in &values {
                                if !Self::validate_value_type(v, column_metadata) {
                                    return Err(format!("Invalid value in IN clause for field '{}': {}", field, v));
                                }
                            }
                        } else {
                            return Err(format!("Invalid IN clause value format for field '{}': {}", field, value));
                        }
                    } else {
                        return Err("Invalid $filter format near token".into());
                    }
                }
                _ => {
                    if let Some(operator) = tokens_iter.next() {
                        if let Some(value) = tokens_iter.next() {
                            if token.contains("/") {
                                let parts: Vec<&str> = token.split('/').collect();
                                if parts.len() >= 2 {
                                    let mut last_entity_metadata = metadata.clone();
                                    for (i, part) in parts.iter().enumerate() {
                                       
                                        // For each part of the path, check if it's a valid relationship or column
                                        if i == parts.len() - 1 {
                                            // Last part should be a field in the related entity
                                            if !last_entity_metadata.columns.contains_key(*part) {
                                                return Err(format!("Invalid field in related entity: {} in filter", *part));
                                            }
                                        } else {
                                            // Intermediate parts should be relationships
                                            if !last_entity_metadata.relationships.contains_key(*part) {
                                                return Err(format!("Invalid related entity: {} in filter", *part));
                                            }

                                            last_entity_metadata = last_entity_metadata.relationships.get(*part).unwrap().related_entity_metadata.clone();
                                        }
                                    }
                                } else {
                                    return Err(format!("Invalid nested field format in filter: {}", token));
                                }
                            } else {
                                if !metadata.columns.contains_key(token) {
                                    return Err(format!("Invalid field in filter: {}", token));
                                }
    
                                let column_metadata = metadata.columns.get(token)
                                    .ok_or_else(|| format!("Field '{}' not found in metadata columns", token))?;
    
                                match operator.as_str() {
                                    "eq" | "ne" | "lt" | "le" | "gt" | "ge" => (),
                                    _ => return Err(format!("Unsupported operator in filter: {}", operator)),
                                }
    
                                if !Self::validate_value_type(value, column_metadata) {
                                    return Err(format!("Invalid value for field '{}': {}", token, value));
                                }
                            }
                        }
                    } else {
                        return Err(format!("Invalid $filter format near token: {}", token));
                    }
                }
            }
        }
    
        Ok(())
    }
    
    fn validate_value_type(value: &str, column_metadata: &ColumnMetadata) -> bool {
        match Self::format_sql_value(Some(value), column_metadata) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn process<T: EntityMetadata>(
        filter: &str,
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {

        let tokens = Self::tokenize_filter(filter)?;

        let mut sql: String = String::new();
        sql.push_str("WHERE\n");

        Self::process_with_tokens::<T>(&tokens, alias_manager, &mut sql)
    }
    
    fn process_with_tokens<T: EntityMetadata>(
        tokens: &[String],
        alias_manager: &mut QueryAliasManager,
        sql: &mut String,
    ) -> Result<String, String> {

        let metadata = T::metadata();
        let mut current_sql = String::new();
    
        let main_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
    
        let mut tokens_iter = tokens.iter().peekable();
    
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "(" => {
                    let mut nested_tokens = Vec::new();
                    let mut parentheses_depth = 1;
    
                    while let Some(inner_token) = tokens_iter.next() {
                        if inner_token == "(" {
                            parentheses_depth += 1;
                        } else if inner_token == ")" {
                            parentheses_depth -= 1;
                            if parentheses_depth == 0 {
                                break;
                            }
                        }
                        nested_tokens.push(inner_token.clone());
                    }
    
                    if parentheses_depth != 0 {
                        return Err("Unbalanced parentheses in filter".into());
                    }
    
                    let nested_query = Self::process_with_tokens::<T>(&nested_tokens, alias_manager, sql)?;
                    
                    current_sql.push_str(&format!("({})", nested_query));
                }
                "and" | "or" => {
                    if !current_sql.is_empty() {
                        sql.push_str(&current_sql);
                        current_sql.clear();
                    }
                    sql.push_str(&format!(" {} ", token.to_uppercase()));
                }
                _ => {
                    if let Some(operator) = tokens_iter.next() {
                        if let Some(value) = tokens_iter.next() {
                            let field = token;
                
                            // Handle relational filters (e.g., Entity1/Entity2/Entity2/Entity2/Entity2/Entity2field)
                            if field.contains("/") {
                                let parts: Vec<&str> = field.split('/').collect();

                                if parts.len() >= 2 {
                                    let mut current_metadata = metadata.clone();
                                    let mut alias_path = Vec::new();
                                    let mut current_entity = parts[0];

                                    // Traverse through the relationship path dynamically
                                    for (i, part) in parts.iter().enumerate() {
                                        if i == parts.len() - 1 {
                                            // Last part should be a valid column in the current entity
                                            if !current_metadata.columns.contains_key(*part) {
                                                return Err(format!("Invalid field in entity '{}': {}", current_metadata.table_name, part));
                                            }
                                        } else {
                                            // Traverse the relationship path dynamically
                                            if let Some(relationship) = current_metadata.relationships.get(*part) {
                                                // Move to the related entity for the next part in the path
                                                alias_path.push(current_entity.to_string());
                                                current_metadata = relationship.related_entity_metadata.clone();
                                                current_entity = *part;
                                            } else {
                                                return Err(format!("Invalid relationship in entity '{}': {}", current_metadata.table_name, current_entity));
                                            }
                                        }
                                    }

                                    // Get the last entity before the field
                                    let last_entity = parts[parts.len() - 2];  // This is the entity, not the property
                                    let last_alias = alias_manager.get_or_create_table_alias(last_entity);

                                    let column_name = parts.last().unwrap();
                                    let column_metadata = current_metadata.columns.get(*column_name).unwrap();

                                    let sql_operator = match operator.as_str() {
                                        "eq" => "=",
                                        "lt" => "<",
                                        "gt" => ">",
                                        "le" => "<=",
                                        "ge" => ">=",
                                        "ne" => "!=",
                                        "in" => "IN",
                                        _ => return Err(format!("Unsupported operator: {}", operator)),
                                    };

                                    // Handle IN clause special case
                                    if sql_operator == "IN" {
                                        let values = value
                                            .trim_matches(|c| c == '(' || c == ')')
                                            .split(',')
                                            .map(|v| v.trim().to_string())
                                            .collect::<Vec<String>>();

                                        if values.is_empty() {
                                            return Err("IN clause must have values.".to_string());
                                        }

                                        let formatted_values = values
                                            .iter()
                                            .map(|v| Self::format_sql_value(Some(v), column_metadata))
                                            .collect::<Result<Vec<String>, _>>()?
                                            .join(", ");

                                            current_sql.push_str(&format!(
                                            "[{}].[{}] IN ({})",
                                            last_alias, column_name, formatted_values
                                        ));
                                    } else {
                                        let sql_value = Self::format_sql_value(Some(value), column_metadata)?;
                                        current_sql.push_str(&format!(
                                            "[{}].[{}] {} {}",
                                            last_alias, column_name, sql_operator, sql_value
                                        ));
                                    }
                                    continue;
                                } else {
                                    return Err(format!("Invalid nested field format in filter: {}", field));
                                }
                            }
                
                            if !metadata.columns.contains_key(field) {
                                return Err(format!("Invalid field in filter: {}", field));
                            }
                
                            let column_metadata = metadata.columns.get(field).unwrap();
                            let sql_operator = match operator.as_str() {
                                "eq" => "=",
                                "lt" => "<",
                                "gt" => ">",
                                "le" => "<=",
                                "ge" => ">=",
                                "ne" => "!=",
                                "in" => "IN",
                                _ => return Err(format!("Unsupported operator: {}", operator)),
                            };
                
                            if sql_operator == "IN" {
                                let values = value
                                    .trim_matches(|c| c == '(' || c == ')')
                                    .split(',')
                                    .map(|v| v.trim().to_string())
                                    .collect::<Vec<String>>();
                
                                if values.is_empty() {
                                    return Err("IN clause must have values.".to_string());
                                }
                
                                let formatted_values = values
                                    .iter()
                                    .map(|v| Self::format_sql_value(Some(v), column_metadata))
                                    .collect::<Result<Vec<String>, _>>()?
                                    .join(", ");
                
                                    current_sql.push_str(&format!(
                                    "[{}].[{}] IN ({})",
                                    main_alias, field, formatted_values
                                ));
                            } else {
                                let sql_value = Self::format_sql_value(Some(value), column_metadata)?;
                                current_sql.push_str(&format!(
                                    "[{}].[{}] {} {}",
                                    main_alias, field, sql_operator, sql_value
                                ));
                            }
                        } else {
                            return Err(format!("Invalid filter format near token: {}", token));
                        }
                    }
                }
            }
        }
    
        if !current_sql.is_empty() {
            sql.push_str(&current_sql);
        }
    
        Ok(sql.clone())
    }
}
