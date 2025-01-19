use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::sqlite::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerMetadata};
use crate::services::query::database::sqlite::common::tokens::select::token::Token;

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

        let sql: String;

        if let Some(text) = text {

            let tokens = match Tokenization::tokenize(text, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };

            sql = match Self::process_with_tokens(&tokens, alias_manager, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };
        }
        else {
            sql = match Self::process_without_tokens(alias_manager, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };  
        }

        // [There no changes in context for while...]

        Ok(sql)
    }
    
    fn process_without_tokens(
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::new();

        let mut properties = Vec::new();

        let metadata = contextualizer.get_context();
           
        sql.push_str("SELECT\n");

        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        for column_metadata in metadata.columns.values() {
            let field_alias: String = alias_manager.get_or_create_field_alias(&column_metadata.column_name());
            
            properties.push(format!(
                "[{}].[{}] AS [{}]",
                table_alias, column_metadata.column_name(), field_alias
            ));
        }

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        // [FROM clause]
        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]", metadata.table_name, table_alias));
        
        Ok(sql.clone())
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::new();

        let mut properties = Vec::new();

        let metadata = contextualizer.get_context();
           
        sql.push_str("SELECT\n");

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

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        // [FROM clause]
        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]", metadata.table_name, table_alias));
        
        Ok(sql.clone())
    }
}