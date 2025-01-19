use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id_authentication_type: i32,
    pub name_authentication_type: String

}

impl Default for Model {
    fn default() -> Self {
        Model {
            id_authentication_type: 0,
            name_authentication_type: "".to_string()
        }
    }
}

impl EntityMetadata for Model {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "id_authentication_type".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "name_authentication_type".to_string(),
                    ColumnMetadata {
                        column_name: "name_authentication_type".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                let relationships = HashMap::new();
               
                EntityDescription {
                    table_name: "AuthenticationType".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
