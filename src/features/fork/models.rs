use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{action::models::Model as ActionModel, topic::models::Model as TopicModel}};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_fork: i32,
    pub id_action: Option<i32>,
    pub id_topic: Option<i32>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_fork: 0,
            id_action: None,
            id_topic: None
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_fork".to_string(),
                    ColumnMetadata {
                        column_name: "id_fork".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "id_action".to_string(),
                    ColumnMetadata {
                        column_name: "id_action".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                columns.insert(
                    "id_topic".to_string(),
                    ColumnMetadata {
                        column_name: "id_topic".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );


                let mut relationships = HashMap::new();
                relationships.insert(
                    "Action".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: ActionModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_action".to_string()],
                        related_keys: vec!["id_action".to_string()],
                    },
                );

                relationships.insert(
                    "Topic".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: TopicModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_topic".to_string()],
                        related_keys: vec!["id_topic".to_string()],
                    },
                );

                EntityDescription {
                    table_name: "Fork".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
