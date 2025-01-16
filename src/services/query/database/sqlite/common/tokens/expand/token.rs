use std::collections::HashMap;

use crate::dto::metadata::{EntityDescription, RelationshipMetadata};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata_dependencies: HashMap<i32, (EntityDescription, RelationshipMetadata)>
    }
}

impl Token {
    pub fn new_property(metadata_dependencies: HashMap<i32, (EntityDescription, RelationshipMetadata)>) -> Self {
        Token::Property {
            metadata_dependencies
        }
    }
}
