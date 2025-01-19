use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata, RelationshipMetadata, RelationshipType}, features::{authentication_type::models::Model as AuthenticationTypeModel, user::models::Model as UserModel}};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_user_authentication_type: i32,
    pub date_user_authentication_type: NaiveDateTime,
    pub active_user_authentication_type: bool,
    pub id_authentication_type: Option<i32>,
    pub id_user: Option<i32>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_user_authentication_type: 0,
            date_user_authentication_type: Utc.with_ymd_and_hms(0,0,0,0,0,0).unwrap().naive_local(),
            active_user_authentication_type: false,
            id_authentication_type: None,
            id_user: None,
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_user_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "id_user_authentication_type".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "date_user_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "date_user_authentication_type".to_string(),
                        column_type: TypeId::of::<NaiveDateTime>(),
                    },
                );

                columns.insert(
                    "active_user_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "active_user_authentication_type".to_string(),
                        column_type: TypeId::of::<bool>(),
                    },
                );

                columns.insert(
                    "id_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "id_authentication_type".to_string(),
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
                    "AuthenticationType".to_string(),
                    RelationshipMetadata {
                        related_entity_metadata: AuthenticationTypeModel::metadata().clone(),
                        relationship_type: RelationshipType::ManyToOne,
                        foreign_keys: vec!["id_authentication_type".to_string()],
                        related_keys: vec!["id_authentication_type".to_string()],
                    },
                );

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
                    table_name: "UserAuthenticationType".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
