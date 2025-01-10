use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::any::TypeId;
use crate::dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata};

#[derive(Serialize, Deserialize)]
pub struct RelationModel {
    pub id_relation: i32,
    pub type_relation: String

}

impl Default for RelationModel {
    fn default() -> Self {
        RelationModel {
            id_relation: 0,
            type_relation: "".to_string()
        }
    }
}

impl EntityMetadata for RelationModel {
    fn metadata() -> &'static EntityDescription {
        lazy_static::lazy_static! {
            static ref METADATA: EntityDescription = {
                let mut columns = HashMap::new();
                columns.insert(
                    "id_relation".to_string(),
                    ColumnMetadata {
                        column_name: "id_relation".to_string(),
                        column_type: TypeId::of::<i32>(),
                    },
                );

                columns.insert(
                    "type_relation".to_string(),
                    ColumnMetadata {
                        column_name: "type_relation".to_string(),
                        column_type: TypeId::of::<String>(),
                    },
                );

                let relationships = HashMap::new();
               
                EntityDescription {
                    table_name: "Relation".to_string(),
                    columns,
                    relationships,
                }
            };
        }
        &METADATA
    }
}
