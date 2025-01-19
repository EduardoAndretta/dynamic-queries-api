use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_role: i32,
    pub name_role: String,
    pub description_role: String

}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_role: 0,
            name_role: "".to_string(),
            description_role: "".to_string()
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_role".to_string(),
                    ColumnMetadata {
                        column_name: "id_role".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_role".to_string(),
                    ColumnMetadata {
                        column_name: "name_role".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                columns.insert(
                    "description_role".to_string(),
                    ColumnMetadata {
                        column_name: "description_role".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                let relationships = HashMap::new();
               
                EntityDescription {
                    table_name: "Role".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
