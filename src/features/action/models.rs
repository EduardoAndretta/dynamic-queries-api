use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{relation::models::Model as RelationModel, topic::models::Model as TopicModel, user::models::Model as UserModel}};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_action: i32,
    pub date_action: NaiveDateTime,
    pub id_user: Option<i32>,
    pub id_relation: Option<i32>,
    pub id_topic: Option<i32>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_action: 0,
            date_action: Utc.with_ymd_and_hms(0,0,0,0,0,0).unwrap().naive_local(),
            id_user: None,
            id_relation: None,
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
                    "id_action".to_string(),
                    ColumnMetadata {
                        column_name: "id_action".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "date_action".to_string(),
                    ColumnMetadata {
                        column_name: "date_action".to_string(),
                        column_type: TypeId::of::<NaiveDateTime>(),
                    },
                );

                columns.insert(
                    "id_user".to_string(),
                    ColumnMetadata {
                        column_name: "id_user".to_string(),
                        column_type: TypeId::of::<Option<i32>>(),
                    },
                );

                columns.insert(
                    "id_relation".to_string(),
                    ColumnMetadata {
                        column_name: "id_relation".to_string(),
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
                    "User".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: UserModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_user".to_string()],
                        related_keys: vec!["id_user".to_string()],
                    },
                );

                relationships.insert(
                    "Relation".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: RelationModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_relation".to_string()],
                        related_keys: vec!["id_relation".to_string()],
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
                    table_name: "Action".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
