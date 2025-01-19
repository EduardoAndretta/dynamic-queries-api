use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::user::models::Model as UserModel};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_user_system: i32,
    pub password_user: String,
    pub id_user: Option<i32>
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_user_system: 0,
            password_user: "".to_string(),
            id_user: None
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_user_system".to_string(),
                    ColumnMetadata {
                        column_name: "id_user_system".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "password_user".to_string(),
                    ColumnMetadata {
                        column_name: "password_user".to_string(),
                        column_type: TypeId::of::<String>(),
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
                    "User".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: UserModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_user".to_string()],
                        related_keys: vec!["id_user".to_string()],
                    },
                );

                EntityDescription {
                    table_name: "UserSystem".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
