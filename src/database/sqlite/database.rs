use std::any::TypeId;
use serde_json::{Map, Value};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Column, Row, TypeInfo};
use chrono::{NaiveDate, NaiveDateTime};

use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::query_params::QueryParams;
use crate::dto::metadata::{ColumnMetadata, EntityMetadata};

use crate::services::query::database::sqlite::query::Query;

use super::common::context::{contextualizer::ContextualizerMetadata, mapping::metadata_mapping::{DynamicStruct, PropertyType}};

pub struct Database(pub SqlitePool);

impl crate::database::Database for Database {
    async fn execute<T: EntityMetadata>(&self, options: &QueryParams) -> Result<Vec<Value>, String> {   
        
        let mut contextualizer = ContextualizerMetadata::new(T::metadata().clone());

        let mut alias_manger = QueryAliasManager::new();

        let query = Query.build_query(options, &mut alias_manger, &mut contextualizer)?;
        
        let rows = sqlx::query(&query)
            .fetch_all(&self.0)
            .await
            .map_err(|e| format!("Error executing query: {}", e))?;

        let properties_hierarchy = Self::parse_properties_hierarchy(&rows, &alias_manger, &mut contextualizer);

        let mut results = Vec::new();
    
        for row in rows {
            let mut json_row = Map::new();
    
            // Parse and populate fields
            Self::parse_and_populate_fields(&row, &properties_hierarchy, &mut json_row);
    
            results.push(Value::Object(json_row));
        }
    
        Ok(results)
    }
    
}

impl Database {
    fn parse_properties_hierarchy(
        rows: &Vec<SqliteRow>,
        alias_manager: &QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> DynamicStruct {
        let metadata = contextualizer.get_context();
    
        let alias_map = alias_manager.get_field_alias();

        let column_names: Vec<String> = if let Some(first_row) = rows.first() {
            first_row.columns().iter()
                .map(|column| {
                    alias_map.iter()
                        .find(|(_, alias)| *alias == column.name())
                        .map(|(nested_name, _)| nested_name.to_string())
                        .unwrap_or_else(|| column.name().to_string())
                })
                .collect()
        } else {
            Vec::new()
        };

        let mut properties_hierarchy = DynamicStruct::new();
    
        let mut added_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for column_name in column_names {
            let parts: Vec<&str> = column_name.split('.').collect();
    
            let real_name = alias_map.get(&column_name).unwrap_or_else(|| &column_name);
    
            if parts.len() == 1 {
                if let Some(column_metadata) = metadata.columns.get(parts[0]) {
                    
                    if added_keys.insert(parts[0].to_string()) {
                        properties_hierarchy.add_property(
                            parts[0],
                            PropertyType::new_property(column_metadata.clone(), real_name.to_string()),
                        );
                    }
                } else if added_keys.insert(parts[0].to_string()) {
                    
                    properties_hierarchy.add_property(
                        parts[0],
                        PropertyType::new_unknow_property(parts[0].to_string(), real_name.to_string()),
                    );
                }
            } else {

                let mut current_metadata = metadata.clone();
                let mut key_path = String::new();
    
                for (i, part) in parts.iter().enumerate() {
                    if i == parts.len() - 1 {

                        let full_path = format!("{}.{}", key_path, part);
                        if let Some(column_metadata) = current_metadata.columns.get(*part) {
                            if added_keys.insert(full_path.clone()) {
                                properties_hierarchy.add_property(
                                    &full_path,
                                    PropertyType::new_property(column_metadata.clone(), real_name.to_string()),
                                );
                            }
                        } else if added_keys.insert(full_path.clone()) {
                            properties_hierarchy.add_property(
                                &full_path,
                                PropertyType::new_unknow_property(part.to_string(), real_name.to_string()),
                            );
                        }
                    } else {

                        key_path = if key_path.is_empty() {
                            part.to_string()
                        } else {
                            format!("{}.{}", key_path, part)
                        };
    
                        if let Some(relation) = current_metadata.relationships.get(*part) {
                            current_metadata = relation.related_entity_metadata.clone();
                        } else {

                            if added_keys.insert(key_path.clone()) {
                                properties_hierarchy.add_property(
                                    &key_path,
                                    PropertyType::new_unknow_property(part.to_string(), real_name.to_string()),
                                );
                            }
                            break;
                        }
    
                        if added_keys.insert(key_path.clone()) {
                            properties_hierarchy.add_property(
                                &key_path,
                                PropertyType::new_relational(current_metadata.clone(), real_name.to_string()),
                            );
                        }
                    }
                }
            }
        }
    
        properties_hierarchy
    }
    
    fn populate_relational_fields(
        row: &SqliteRow,
        hierarchy: &std::collections::HashMap<String, PropertyType>,
        json_row: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        for (key, property) in hierarchy {
            match property {
                PropertyType::Property { metadata, key } => {
                    
                    // [Parsed Metadata | The name of metadata will not the same for database]
                    let parsed_metadata = ColumnMetadata {
                        column_name: key.to_string(),
                        column_type: metadata.column_type
                    };

                    if let Some(value) = Self::fetch_column_value(row, &parsed_metadata) {
                        json_row.insert(metadata.column_name.to_string(), value);
                    }
                }
                PropertyType::RelationalProperty { properties, .. } => { 

                    let mut nested_json: Map<String, Value> = serde_json::Map::new();

                    Self::populate_relational_fields(row, properties, &mut nested_json);
                    
                    if !nested_json.is_empty() {
                        json_row.insert(key.clone(), serde_json::Value::Object(nested_json));
                    }
                }
                PropertyType::UnknowProperty { key, .. } => {

                    if let Some(value) = Self::fetch_column_value_with_row(row, key) {
                        json_row.insert(key.clone(), value);
                    }
                }
            }
        }
    }
    
    fn parse_and_populate_fields(
        row: &SqliteRow,
        hierarchy: &DynamicStruct,
        json_row: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        for (key, property) in &hierarchy.fields {
            match property {
                PropertyType::Property { metadata, key } => {
                    
                    // [Parsed Metadata | The name of metadata will not the same for database]
                    let parsed_metadata = ColumnMetadata {
                        column_name: key.to_string(),
                        column_type: metadata.column_type
                    };

                    if let Some(value) = Self::fetch_column_value(row, &parsed_metadata) {
                        json_row.insert(metadata.column_name.to_string(), value);
                    }
                }
                PropertyType::RelationalProperty { properties, .. } => {

                    let mut nested_json: Map<String, Value> = serde_json::Map::new();
                    
                    Self::populate_relational_fields(row, properties, &mut nested_json);
                    
                    if !nested_json.is_empty() {
                        json_row.insert(key.clone(), serde_json::Value::Object(nested_json));
                    }
                }
                PropertyType::UnknowProperty { key , ..} => {
                    
                    if let Some(value) = Self::fetch_column_value_with_row(row, key) {                       
                        json_row.insert(key.clone(), value);
                    }
                }
            }
        }
    }
     
    fn fetch_column_value_with_row(row: &SqliteRow, column_name: &str) -> Option<serde_json::Value> {
        
        // [Get the column type name as a String]
        let column_type = row
            .columns()
            .iter()
            .find(|c| c.name() == column_name)
            .map(|c| c.type_info().name());
    
        // [Now match directly against the type name]
        match column_type.as_deref() {
            Some("INTEGER") => {
                row.try_get::<i64, _>(column_name)
                    .ok()
                    .map(|v| serde_json::Value::Number(serde_json::Number::from(v)))
            }
            Some("REAL") | Some("FLOAT") | Some("NUMERIC") | Some("DECIMAL") => {
                row.try_get::<f64, _>(column_name)
                    .ok()
                    .map(|v| serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap()))
            }
            Some("TEXT") => row.try_get::<String, _>(column_name).ok().map(serde_json::Value::String),
            Some("BOOLEAN") => {
                row.try_get::<bool, _>(column_name)
                    .ok()
                    .map(|v| serde_json::Value::Bool(v))
            }
            Some("DATE") | Some("DATETIME") => row.try_get::<String, _>(column_name).ok().map(serde_json::Value::String),
            _ => {
                eprintln!("Warning: Unsupported column type for {}", column_name);
                None
            }
        }
    }

    fn fetch_column_value(row: &SqliteRow, column: &ColumnMetadata) -> Option<Value> {
        match column.column_type {
            _ if column.column_type == TypeId::of::<Option<i32>>() => {
                row.try_get::<Option<i32>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(|v| Value::Number(serde_json::Number::from(v as i64))))
            }
            _ if column.column_type == TypeId::of::<Option<i64>>() => {
                row.try_get::<Option<i64>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(|v| Value::Number(serde_json::Number::from(v))))
            }
            _ if column.column_type == TypeId::of::<Option<f32>>() => {
                row.try_get::<Option<f32>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(|v| Value::Number(serde_json::Number::from_f64(v as f64).unwrap())))
            }
            _ if column.column_type == TypeId::of::<Option<f64>>() => {
                row.try_get::<Option<f64>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(|v| Value::Number(serde_json::Number::from_f64(v).unwrap())))
            }
            _ if column.column_type == TypeId::of::<Option<bool>>() => {
                row.try_get::<Option<bool>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(Value::Bool))
            }
            _ if column.column_type == TypeId::of::<Option<String>>() => {
                row.try_get::<Option<String>, _>(&*column.column_name)
                    .ok()
                    .and_then(|opt_v| opt_v.map(Value::String))
            }
            _ if column.column_type == TypeId::of::<i32>() => {
                row.try_get::<i32, _>(&*column.column_name)
                    .ok()
                    .map(|v| Value::Number(serde_json::Number::from(v as i64)))
            }
            _ if column.column_type == TypeId::of::<i64>() => {
                row.try_get::<i64, _>(&*column.column_name)
                    .ok()
                    .map(|v| Value::Number(serde_json::Number::from(v)))
            }
            _ if column.column_type == TypeId::of::<f32>() => {
                row.try_get::<f32, _>(&*column.column_name)
                    .ok()
                    .map(|v| Value::Number(serde_json::Number::from_f64(v as f64).unwrap()))
            }
            _ if column.column_type == TypeId::of::<f64>() => {
                row.try_get::<f64, _>(&*column.column_name)
                    .ok()
                    .map(|v| Value::Number(serde_json::Number::from_f64(v).unwrap()))
            }
            _ if column.column_type == TypeId::of::<bool>() => {
                row.try_get::<bool, _>(&*column.column_name)
                    .ok()
                    .map(|v| Value::Bool(v))
            }
            _ if column.column_type == TypeId::of::<String>() => {
                row.try_get::<String, _>(&*column.column_name)
                    .ok()
                    .map(Value::String)
            }
            _ if column.column_type == TypeId::of::<NaiveDate>() => {
                row.try_get::<String, _>(&*column.column_name)
                    .ok()
                    .and_then(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok())
                    .map(|date| Value::String(date.to_string()))
            }
            _ if column.column_type == TypeId::of::<NaiveDateTime>() => {
                row.try_get::<String, _>(&*column.column_name)
                    .ok()
                    .and_then(|v| NaiveDateTime::parse_from_str(&v, "%Y-%m-%dT%H:%M:%S").ok())
                    .map(|datetime| Value::String(datetime.to_string()))
            }
            _ => {
                eprintln!("Warning: Unsupported column type for {}", column.column_name);
                None
            }
        }
    }
}