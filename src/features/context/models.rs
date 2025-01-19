use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_context: i32,
    pub name_context: String

}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_context: 0,
            name_context: "".to_string()
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_context".to_string(),
                    ColumnMetadata {
                        column_name: "id_context".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_context".to_string(),
                    ColumnMetadata {
                        column_name: "name_context".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                let relationships = HashMap::new();
               
                EntityDescription {
                    table_name: "Context".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
