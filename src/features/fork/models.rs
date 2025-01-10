use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{action::models::ActionModel, topic::models::TopicModel}};

#[derive(Serialize, Deserialize)]
pub struct ForkModel {
    pub id_fork: i32,
    pub id_action: Option<i32>,
    pub id_topic: Option<i32>
}

impl Default for ForkModel {
    fn default() -> Self {
        ForkModel {
            id_fork: 0,
            id_action: None,
            id_topic: None
        }
    }
}

impl EntityMetadata for ForkModel {
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
                        related_entity: "Action".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_action".to_string()],
                        related_keys: vec!["id_action".to_string()],
                        navigation_property: "Action".to_string(),
                    },
                );

                relationships.insert(
                    "Topic".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: TopicModel::metadata().clone(),
                        related_entity: "Topic".to_string(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_topic".to_string()],
                        related_keys: vec!["id_topic".to_string()],
                        navigation_property: "Topic".to_string(),
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
