use crate::dto::metadata::{ColumnMetadata, EntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata: EntityDescription,
        column_metadata: ColumnMetadata,
        relation_path: String
    }
}

impl Token {
    pub fn new_property(metadata: EntityDescription, column_metadata: ColumnMetadata, relation_path: String) -> Self {
        Token::Property {
            metadata,
            column_metadata,
            relation_path
        }
    }
}
