use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::mssql::common::context::contextualizer::{ContextualizerMetadata, ContextualizerColumnMetadata};
use crate::services::query::database::mssql::common::tokens::orderby::token::Token;

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

        let mut sql: String = String::from("");

        if let Some(text) = text {
            let tokens = match Tokenization::tokenize(text, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };

            sql = match Self::process_with_tokens(&tokens, alias_manager) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };
        }

        // [There no changes in context for while...]

        Ok(sql)
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();

        let mut properties = Vec::new();
           
        sql.push_str("ORDER BY\n");

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

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        Ok(sql.clone())
    }

}