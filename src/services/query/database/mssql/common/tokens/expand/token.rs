use std::collections::HashMap;

use crate::database::mssql::common::context::contextualizer::{ContextualizerRelationshipMetadata, ContextualizerEntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata_dependencies: HashMap<i32, (ContextualizerEntityDescription, ContextualizerRelationshipMetadata)>
    }
}

impl Token {
    pub fn new_property(metadata_dependencies: HashMap<i32, (ContextualizerEntityDescription, ContextualizerRelationshipMetadata)>) -> Self {
        Token::Property {
            metadata_dependencies
        }
    }
}
