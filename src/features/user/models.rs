use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_user: i32,
    pub name_user: String,
    pub date_user: NaiveDateTime
}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_user: 0,
            name_user: "".to_string(),
            date_user: Utc.with_ymd_and_hms(0,0,0,0,0,0).unwrap().naive_local(),
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_user".to_string(),
                    ColumnMetadata {
                        column_name: "id_user".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_user".to_string(),
                    ColumnMetadata {
                        column_name: "name_user".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                columns.insert(
                    "date_user".to_string(),
                    ColumnMetadata {
                        column_name: "date_user".to_string(),
                        column_type: TypeId::of::<NaiveDateTime>(),
                    },
                );

                let relationships = HashMap::new();
                
                EntityDescription {
                    table_name: "User".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
