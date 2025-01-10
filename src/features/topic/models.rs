use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{context::models::ContextModel, user::models::UserModel}};

#[derive(Serialize, Deserialize)]
pub struct TopicModel {
    pub id_topic: i32,
    pub name_topic: String,
    pub id_context: Option<i32>,
    pub id_user: Option<i32>,
}

impl Default for TopicModel {
    fn default() -> Self {
        TopicModel {
            id_topic: 0,
            name_topic: "".to_string(),
            id_context: None,
            id_user: None
        }
    }
}

impl EntityMetadata for TopicModel {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_topic".to_string(),
                    ColumnMetadata {
                        column_name: "id_topic".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_topic".to_string(),
                    ColumnMetadata {
                        column_name: "name_topic".to_string(),
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

                columns.insert(
                    "id_user".to_string(),
                    ColumnMetadata {
                        column_name: "id_user".to_string(),
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

                relationships.insert(
                    "User".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: UserModel::metadata().clone(),
                        related_entity: "User".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_user".to_string()],
                        related_keys: vec!["id_user".to_string()],
                        navigation_property: "User".to_string(),
                    },
                );

                EntityDescription {
                    table_name: "Topic".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
