use std::{any::TypeId, collections::HashMap};

use crate::dto::metadata::{EntityDescription, RelationshipMetadata, RelationshipType};

#[derive(Debug, Clone)]
pub struct ContextualizerMetadata {
    metadata: ContextualizerEntityDescription,
}

impl ContextualizerMetadata {
    pub fn new(metadata: EntityDescription) -> Self {
        let columns = metadata.columns.into_iter().map(|(k, v)| {
            (k, ContextualizerColumnMetadata {
                column_name: v.column_name,
                column_type: v.column_type,
            })
        }).collect::<HashMap<_, _>>();

        let relationships = metadata.relationships.into_iter().map(|(k, v)| {
            (k, convert_to_contextualizer_relationship_metadata(&v))
        }).collect::<HashMap<_, _>>();

        Self {
            metadata: ContextualizerEntityDescription {
                table_name: metadata.table_name,
                columns,
                relationships,
            },
        }
    }

    pub fn update_context(&mut self, metadata: ContextualizerEntityDescription) {
        self.metadata = metadata;
    }

    pub fn get_context(&self) -> ContextualizerEntityDescription {
        self.metadata.clone()
    }
}

fn convert_to_contextualizer_entity(entity: &EntityDescription) -> ContextualizerEntityDescription {
    let columns = entity.columns.iter().map(|(k, v)| {
        (k.clone(), ContextualizerColumnMetadata {
            column_name: v.column_name.clone(),
            column_type: v.column_type,
        })
    }).collect::<HashMap<_, _>>();

    let relationships = entity.relationships.iter().map(|(k, v)| {
        (k.clone(), convert_to_contextualizer_relationship_metadata(&v))
    }).collect::<HashMap<_, _>>();

    ContextualizerEntityDescription {
        table_name: entity.table_name.clone(),
        columns,
        relationships,
    }
}

fn convert_to_contextualizer_relationship_metadata(metadata: &RelationshipMetadata) -> ContextualizerRelationshipMetadata {
    ContextualizerRelationshipMetadata {
        related_entity_metadata: convert_to_contextualizer_entity(&metadata.related_entity_metadata),
        relationship_type: convert_to_contextualizer_relationship_type(&metadata.relationship_type),
        foreign_keys: metadata.foreign_keys.clone(),
        related_keys: metadata.related_keys.clone(),
        expanded: false, 
    }
}

fn convert_to_contextualizer_relationship_type(relationship_type: &RelationshipType) -> ContextualizerRelationshipType {
    match relationship_type {
        RelationshipType::ManyToMany => ContextualizerRelationshipType::ManyToMany,
        RelationshipType::ManyToOne  => ContextualizerRelationshipType::ManyToOne,
        RelationshipType::OneToMany  => ContextualizerRelationshipType::OneToMany,
        RelationshipType::OneToOne   => ContextualizerRelationshipType::OneToOne,
    }
}

#[derive(Debug, Clone)]
pub struct ContextualizerColumnMetadata {
    pub column_name: String,
    pub column_type: TypeId,
}

impl Default for ContextualizerColumnMetadata {
    fn default() -> Self {
        ContextualizerColumnMetadata {
            column_name: "".to_string(),
            column_type: TypeId::of::<String>(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContextualizerRelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone)]
pub struct ContextualizerRelationshipMetadata {
    pub related_entity_metadata: ContextualizerEntityDescription,
    pub relationship_type: ContextualizerRelationshipType,
    pub foreign_keys: Vec<String>,
    pub related_keys: Vec<String>,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub struct ContextualizerEntityDescription {
    pub table_name: String,
    pub columns: HashMap<String, ContextualizerColumnMetadata>,
    pub relationships: HashMap<String, ContextualizerRelationshipMetadata>,
}
