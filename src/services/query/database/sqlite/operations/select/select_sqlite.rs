use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;

use crate::services::query::database::sqlite::common::tokens::select::token::Token;

use super::token::tokenization::Tokenization;

pub struct SelectSqlite;

impl SelectSqlite {
    pub fn process<T: EntityMetadata>(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {

        if let Some(text) = &text {
            let tokens = Tokenization::tokenize::<T>(text)?;

            return Self::process_with_tokens::<T>(&tokens, alias_manager)
        }
        Self::process_without_tokens::<T>(alias_manager)  
    }
    
    fn process_without_tokens<T : EntityMetadata>(
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let metadata = T::metadata();

        let mut sql: String = String::new();

        let mut properties = Vec::new();
           
        sql.push_str("SELECT\n");

        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        for column_metadata in metadata.columns.values() {
            let field_alias: String = alias_manager.get_or_create_field_alias(&column_metadata.column_name);
            
            properties.push(format!(
                "[{}].[{}] AS [{}]",
                table_alias, column_metadata.column_name, field_alias
            ));
        }

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        // [FROM clause]
        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]\n", metadata.table_name, table_alias));
        
        Ok(sql.clone())
    }

    fn process_with_tokens<T : EntityMetadata>(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let metadata = T::metadata();

        let mut sql: String = String::new();

        let mut properties = Vec::new();
           
        sql.push_str("SELECT\n");

        for token in tokens {
            match token {
                Token::Property { metadata, column_metadata, relation_path} => {
                    let table_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
                    let field_alias = alias_manager.get_or_create_field_alias(&relation_path);
                    
                    properties.push(format!(
                        "[{}].[{}] AS [{}]", 
                        table_alias, column_metadata.column_name, field_alias
                    ));
                },
            }
        }

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        // [FROM clause]
        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]\n", metadata.table_name, table_alias));
        
        Ok(sql.clone())
    }
}