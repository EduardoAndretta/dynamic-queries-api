use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_permission: i32,
    pub name_permission: String,
    pub description_permission: String

}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_permission: 0,
            name_permission: "".to_string(),
            description_permission: "".to_string()
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_permission".to_string(),
                    ColumnMetadata {
                        column_name: "id_permission".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_permission".to_string(),
                    ColumnMetadata {
                        column_name: "name_permission".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                columns.insert(
                    "description_permission".to_string(),
                    ColumnMetadata {
                        column_name: "description_permission".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                let relationships = HashMap::new();
               
                EntityDescription {
                    table_name: "Permission".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
