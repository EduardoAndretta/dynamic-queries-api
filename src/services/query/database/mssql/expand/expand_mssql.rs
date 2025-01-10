use crate::dto::metadata::EntityDescription;
use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::metadata::EntityMetadata;

pub struct ExpandMssql;

impl ExpandMssql {
    pub fn validate<T: EntityMetadata>(
        expand: &str
    ) -> Result<(), String> {

        if expand.trim().is_empty() {
            return Err("Invalid $expand: cannot be empty".into());
        }
                
        let metadata = T::metadata();

        let expand_fields = expand.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>();
        for field in &expand_fields {
            let parts: Vec<&str> = field.split('/').collect();
            let mut current_relationship = metadata;
    
            for part in &parts {
                if !current_relationship.relationships.contains_key(*part) {
                    return Err(format!("Invalid $expand relationship: {}", field));
                }
                current_relationship = &current_relationship.relationships.get(*part).unwrap().related_entity_metadata;
            }
        }
        Ok(())
    }
    
    pub fn process<T: EntityMetadata>(
        expand: &str,
        alias_manager: &mut QueryAliasManager
    ) -> Result<String, String> {

        println!("Processing $expand: {}", expand);

        let metadata = T::metadata();
        let mut sql = String::new();
    
        Self::process_relationship(expand, alias_manager, &metadata, &mut sql)?;
    
        Ok(sql)
    }

    pub fn process_relationship(
        expand: &str,
        alias_manager: &mut QueryAliasManager,
        metadata: &EntityDescription,
        sql: &mut String,
    ) -> Result<(), String> {
        for relationship_path in expand.split(',') {
            let relationship_path = relationship_path.trim();
            if relationship_path.is_empty() {
                return Err("Expand path contains an empty relationship.".to_string());
            }
    
            let mut relationship_parts = relationship_path.split('/');
            let first_relationship = relationship_parts
                .next()
                .ok_or_else(|| "Invalid expand path: no relationship found.".to_string())?;
    
            let rel_metadata = metadata
                .relationships
                .get(first_relationship)
                .ok_or_else(|| format!("Invalid $expand relationship: {}", first_relationship))?;
    
            let main_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
            let related_alias = alias_manager.get_or_create_table_alias(&rel_metadata.related_entity);
    
            let join_conditions: Vec<String> = rel_metadata
                .foreign_keys
                .iter()
                .zip(rel_metadata.related_keys.iter())
                .map(|(fk, rk)| format!("[{}].[{}] = [{}].[{}]", main_alias, fk, related_alias, rk))
                .collect();
    
            if join_conditions.is_empty() {
                return Err(format!(
                    "No foreign key mappings found for relationship: {}",
                    first_relationship
                ));
            }
    
            sql.push_str(&format!(
                "LEFT JOIN [{}] AS [{}] ON {}\n",
                rel_metadata.related_entity,
                related_alias,
                join_conditions.join(" AND ")
            ));
    
            let remaining_path: String = relationship_parts.collect::<Vec<&str>>().join("/");
            if !remaining_path.is_empty() {
                Self::process_relationship(
                    &remaining_path,
                    alias_manager,
                    &rel_metadata.related_entity_metadata,
                    sql,
                )?;
            }
        }
    
        Ok(())
    }
}