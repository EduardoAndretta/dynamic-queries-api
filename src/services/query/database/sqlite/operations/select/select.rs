use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::sqlite::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerMetadata};
use crate::services::query::database::sqlite::common::{tokens::select::token::Token, internal_specification::select::internal_specification::InternalSpecification};

use super::{token::tokenization::Tokenization, compute::compute::Compute};

pub struct Select;

impl Select {
    pub fn process(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let error = |message: String| -> String {
            format!("Invalid $select: {}", message)
        };

        let mut properties: Vec<String> = Vec::new();

        if !contextualizer.ignore_rules.select {
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
            else {
                properties.extend(
                    match Self::process_without_tokens(alias_manager, contextualizer) {
                        Err(err) => return Err(error(err)),
                        Ok(value) => value
                    });
            }
        }

        // [internal-specification]
        match InternalSpecification::process_internal_specification(&mut properties, alias_manager, contextualizer) {
            Err(err) => return Err(error(err)),
            Ok(value) => value
        };

        let mut sql = String::new();

        if !properties.is_empty() {

            sql = match Self::build_string_statement(&properties, alias_manager, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };
        }

        Ok(sql)
    }
    
    fn build_string_statement(
        properties: &Vec<String>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::new();

        sql.push_str("SELECT\n");

        sql.push_str(&properties.join(", "));

        // [FROM clause]
        let metadata = contextualizer.get_context();
           
        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        sql.push_str("\nFROM\n");
        sql.push_str(&format!("[{}] AS [{}]", metadata.table_name, table_alias));

        Ok(sql)
    }

    fn process_without_tokens(
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<Vec<String>, String> {

        let mut properties = Vec::new();

        let metadata = contextualizer.get_context();
           
        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        for column_metadata in metadata.columns.values() {
            match column_metadata {
                ContextualizerColumnMetadata::Original { column_name, .. } => {
                    let field_alias: String = alias_manager.get_or_create_field_alias(&column_name);
            
                    properties.push(format!(
                        "[{}].[{}] AS [{}]",
                        table_alias, column_name, field_alias
                    ));
                },
                ContextualizerColumnMetadata::Dynamic { .. } => { 
                    Compute::process_computed_column(&column_metadata, &mut properties, alias_manager)?
                } 
            }  
        }

        Ok(properties)
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager
    ) -> Result<Vec<String>, String> {

        let mut properties = Vec::new();
           
        for token in tokens {
            match token {
                Token::Property { metadata, column_metadata, relation_path} => {
                    match column_metadata {
                        ContextualizerColumnMetadata::Original { column_name, .. } => {

                            let table_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
                            let field_alias = alias_manager.get_or_create_field_alias(&relation_path);
                            
                            properties.push(format!(
                                "[{}].[{}] AS [{}]", 
                                table_alias, column_name, field_alias
                            ));
                        },
                        ContextualizerColumnMetadata::Dynamic { .. } => { 
                            Compute::process_computed_column(&column_metadata, &mut properties, alias_manager)?
                        } 
                    }
                }
            }
        }

        Ok(properties)
    }
}