use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::sqlite::common::context::contextualizer::{ContextualizerMetadata, ContextualizerColumnMetadata};
use crate::services::query::database::sqlite::common::tokens::orderby::token::Token;

use super::{compute::compute::Compute, token::tokenization::Tokenization};

pub struct Orderby;

impl Orderby {
    pub fn process(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let error = |message: String| -> String {
            format!("Invalid $orderby: {}", message)
        };

        let mut properties: Vec<String> = Vec::new();

        // [Handle the Internal Specification here]

        if !contextualizer.ignore_rules.orderby {
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

        sql.push_str("ORDER BY\n");

        sql.push_str(&properties.join(", "));

        Ok(sql)
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<Vec<String>, String> {
        
        let mut properties: Vec<String> = Vec::new();

        for token in tokens {
            match token {
                Token::Property { metadata, column_metadata } => {
                    match column_metadata {
                        ContextualizerColumnMetadata::Original { column_name, .. } => {

                            let table_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
                    
                            properties.push(format!(
                                "[{}].[{}]",
                                table_alias, column_name
                            ));
                        },
                        ContextualizerColumnMetadata::Dynamic { .. } => { 
                            Compute::process_computed_column(&column_metadata, &mut properties, alias_manager)?
                        } 
                    }
                },
            }
        }

        Ok(properties)
    }

}