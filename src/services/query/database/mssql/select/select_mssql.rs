use crate::dto::metadata::EntityDescription;
use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;

pub struct SelectMssql;

impl SelectMssql {

    pub fn validate<T: EntityMetadata>(
        select: &str
    ) -> Result<(), String> {
        
        if select.trim().is_empty() {
            return Err("Invalid $select: cannot be empty".into());
        }

        let metadata = T::metadata();

        let select_fields = select.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>();
        for field in &select_fields {
    
            if field.contains('/') {
                let mut parts = field.split('/');
                let mut current_relationship = metadata;
    
                while let Some(relationship) = parts.next() {
                    
                    let is_last_part = parts.clone().count() == 0;
    
                    if !is_last_part {
                        if let Some(related_metadata) = current_relationship.relationships.get(relationship) {
                            current_relationship = &related_metadata.related_entity_metadata;
                        } else {
                            return Err(format!("Invalid relationship in $select field: {}", field));
                        }
                    } else {
                        if !current_relationship.columns.contains_key(relationship) {
                            return Err(format!("Invalid $select field: {}", field));
                        }
                    }
                }
            } else {
                if !metadata.columns.contains_key(field) {
                    return Err(format!("Invalid $select field: {}", field));
                }
            }
        }
        Ok(())
    }
    
    pub fn process<T: EntityMetadata>(
        select: Option<String>,
        query_alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let metadata = T::metadata();
        let mut sql = String::new();
        let mut select_fields = Vec::new();
    
        let main_alias = query_alias_manager.get_or_create_table_alias(&metadata.table_name);
    
        sql.push_str("SELECT\n");
    
        if let Some(select_str) = select {
            for field in select_str.split(',') {
                let field = field.trim();
    
                if let Some(pos) = field.find('/') {
                    let (relationship_path, column) = field.split_at(pos);
                    let column = &column[1..];
    
                    // Handle nested relationships
                    let related_field = Self::process_related_field(
                        relationship_path,
                        column,
                        query_alias_manager,
                        &metadata,
                        relationship_path,
                    )?;
                    select_fields.push(related_field);
                } else {
                    // Handle direct columns
                    if let Some(col_metadata) = metadata.columns.get(field) {
                        let alias = query_alias_manager.get_or_create_field_alias(field);
                        select_fields.push(format!(
                            "[{}].[{}] AS [{}]",
                            main_alias, col_metadata.column_name, alias
                        ));
                    } else {
                        return Err(format!("Invalid field in $select: {}", field));
                    }
                }
            }
        } else {
            println!("No $select provided, using all columns.");
            for col_metadata in metadata.columns.values() {
                let alias = query_alias_manager.get_or_create_field_alias(&col_metadata.column_name);
                select_fields.push(format!(
                    "[{}].[{}] AS [{}]",
                    main_alias, col_metadata.column_name, alias
                ));
            }
        }
    
        sql.push_str(&select_fields.join(",\n    "));
        sql.push_str("\n");
    
        // FROM clause
        sql.push_str("FROM\n");
        sql.push_str(&format!("[{}] AS [{}]\n", metadata.table_name, main_alias));
    
        Ok(sql)
    }
    
    fn process_related_field(
        relationship_path: &str,
        column: &str,
        query_alias_manager: &mut QueryAliasManager,
        metadata: &EntityDescription,
        full_relationship_path: &str,
    ) -> Result<String, String> {
        let mut column_parts = column.split('/');
        let first_column_part = column_parts.next().ok_or_else(|| "Invalid column path")?;
    
        if let Some(relationship_metadata) = metadata.relationships.get(relationship_path) {
            if column_parts.clone().count() == 0 {
                if let Some(col_metadata) =
                    relationship_metadata.related_entity_metadata.columns.get(first_column_part)
                {
                    let relational_alias = query_alias_manager.get_or_create_table_alias(relationship_path);
                    let alias = format!("{}.{}", full_relationship_path, first_column_part);
    
                    // Generate a short alias for the field
                    let field_alias = query_alias_manager.get_or_create_field_alias(&alias);
    
                    return Ok(format!(
                        "[{}].[{}] AS [{}]",
                        relational_alias, col_metadata.column_name, field_alias
                    ));
                } else {
                    return Err(format!(
                        "Invalid field in $select: Column '{}' not found in related table '{}'.",
                        first_column_part, relationship_path
                    ));
                }
            } else {
                let next_column_path = column_parts.collect::<Vec<_>>().join("/");
                let next_relationship = first_column_part;
                let next_relationship_metadata = &relationship_metadata.related_entity_metadata;
                let accumulated_path = format!("{}.{}", full_relationship_path, next_relationship);
    
                return Self::process_related_field(
                    next_relationship,
                    next_column_path.as_str(),
                    query_alias_manager,
                    next_relationship_metadata,
                    &accumulated_path,
                );
            }
        } else {
            return Err(format!(
                "Invalid relationship in $select: Related table '{}' not found in relationships.",
                relationship_path
            ));
        }
    }
}