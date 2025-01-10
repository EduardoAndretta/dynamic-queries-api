use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::context::models::ContextModel};

#[derive(Serialize, Deserialize)]
pub struct ForumModel {
    pub id_forum: i32,
    pub name_forum : String,
    pub description_forum : String,
    pub id_context: Option<i32>
}

impl Default for ForumModel {
    fn default() -> Self {
        ForumModel {
            id_forum: 0,
            name_forum : "".to_string(),
            description_forum : "".to_string(),
            id_context: None
        }
    }
}

impl EntityMetadata for ForumModel {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_forum".to_string(),
                    ColumnMetadata {
                        column_name: "id_forum".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_forum".to_string(),
                    ColumnMetadata {
                        column_name: "name_forum".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                columns.insert(
                    "description_forum".to_string(),
                    ColumnMetadata {
                        column_name: "description_forum".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                columns.insert(
                    "id_context".to_string(),
                    ColumnMetadata {
                        column_name: "id_context".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                let mut relationships = HashMap::new();
                relationships.insert(
                    "Context".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: ContextModel::metadata().clone(),
                        related_entity: "Context".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_context".to_string()],
                        related_keys: vec!["id_context".to_string()],
                        navigation_property: "Context".to_string(),
                    },
                );

                EntityDescription {
                    table_name: "Forum".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
