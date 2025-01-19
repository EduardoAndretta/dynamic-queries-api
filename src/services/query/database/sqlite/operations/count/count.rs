use crate::{database::sqlite::common::context::contextualizer::ContextualizerMetadata, services::query::common::alias_manager::QueryAliasManager};

use super::context::context::Context;

pub struct Count;

impl Count {
    pub fn process(
        alias_manager: &mut QueryAliasManager,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<String, String> {

        let error = |message: String| -> String {
            format!("Invalid $count: {}", message)
        };

        let sql = match Self::process_select(alias_manager, contextualizer) {
            Err(err) => return Err(error(err)),
            Ok(value) => value
        };

        match Context::resolve_context(contextualizer) {
            Err(err) => return Err(error(err)),
            Ok(value) => value
        };

        Ok(sql)
    }

    pub fn process_select(
        alias_manager: &mut QueryAliasManager,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::from("");

        let metadata = contextualizer.get_context();

        sql.push_str("SELECT\n");

        let table_alias: String = alias_manager.get_or_create_table_alias(&metadata.table_name);

        let column_name = "count".to_string();

        let field_alias: String = alias_manager.get_or_create_field_alias(&column_name);
            
        sql.push_str(&format!(
            "COUNT(*) AS [{}]",
            field_alias
        ));

        sql.push_str("\n");

        // [FROM clause]
        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]", metadata.table_name, table_alias));
        
        Ok(sql)
    }
}