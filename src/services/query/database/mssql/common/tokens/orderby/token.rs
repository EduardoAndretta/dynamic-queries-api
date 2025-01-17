use crate::database::mssql::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerEntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata: ContextualizerEntityDescription,
        column_metadata: ContextualizerColumnMetadata
    }
}

impl Token {
    pub fn new_property(metadata: ContextualizerEntityDescription, column_metadata: ContextualizerColumnMetadata) -> Self {
        Token::Property {
            metadata,
            column_metadata
        }
    }
}
