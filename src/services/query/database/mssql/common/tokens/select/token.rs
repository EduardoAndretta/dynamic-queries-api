use crate::database::mssql::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerEntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata: ContextualizerEntityDescription,
        column_metadata: ContextualizerColumnMetadata,
        relation_path: String
    }
}

impl Token {
    pub fn new_property(metadata: ContextualizerEntityDescription, column_metadata: ContextualizerColumnMetadata, relation_path: String) -> Self {
        Token::Property {
            metadata,
            column_metadata,
            relation_path
        }
    }
}
