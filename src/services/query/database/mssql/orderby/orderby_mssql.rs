use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;

pub struct OrderbyMssql;

impl OrderbyMssql {

    pub fn validate<T: EntityMetadata>(
        orderby: &str,
    ) -> Result<(), String> {

        if orderby.trim().is_empty() {
            return Err("Invalid $orderby: cannot be empty".into());
        }

        let metadata = T::metadata();
    
        let orderby_fields = orderby.split(',').map(|o| o.trim().to_string()).collect::<Vec<_>>();
        for field in &orderby_fields {

            let parts: Vec<&str> = field.split('/').collect();
            let mut current_metadata = metadata;
    
            for (i, part) in parts.iter().enumerate() {
                if i == parts.len() - 1 {

                    if !current_metadata.columns.contains_key(*part) {
                        return Err(format!("Invalid $orderby column: {}", part));
                    }
                } else {

                    if let Some(relationship) = current_metadata.relationships.get(*part) {
                        current_metadata = &relationship.related_entity_metadata;
                    } else {
                        return Err(format!("Invalid $orderby relationship: {}", part));
                    }
                }
            }
        }
        Ok(())
    }
    
    pub fn process<T: EntityMetadata>(
        orderby: &str,
        query_alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let metadata = T::metadata();
        let mut sql = String::new();
        let main_alias = query_alias_manager.get_or_create_table_alias(&metadata.table_name);
    
        let orderby_fields = orderby
            .split(',')
            .map(|field| {
                let parts: Vec<&str> = field.trim().split('/').collect();
                if parts.len() > 1 {

                    let mut current_metadata = metadata;
                    let mut current_alias = main_alias.clone();
    
                    for table in parts.iter().take(parts.len() - 1) {
                        if let Some(relationship) = current_metadata.relationships.get(*table) {
                            current_metadata = &relationship.related_entity_metadata;
                            current_alias = query_alias_manager.get_or_create_table_alias(&current_metadata.table_name);
                        } else {
                            return Err(format!("Relationship '{}' does not exist in metadata.", table));
                        }
                    }
    
                    let column_name = parts.last().unwrap();
                    current_metadata
                        .columns
                        .get(*column_name)
                        .map(|col| format!("[{}].[{}]", current_alias, col.column_name))
                        .ok_or_else(|| format!("Column '{}' does not exist in metadata.", column_name))
                } else {

                    metadata
                        .columns
                        .get(field.trim())
                        .map(|col| format!("[{}].[{}]", main_alias, col.column_name))
                        .ok_or_else(|| format!("Column '{}' does not exist in metadata.", field.trim()))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
    
        if !orderby_fields.is_empty() {
            sql.push_str("ORDER BY\n");
            sql.push_str(&orderby_fields.join(",\n    "));
            sql.push_str("\n");
        }
    
        Ok(sql)
    }
}