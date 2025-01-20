use std::{any::TypeId, collections::HashMap};

use crate::dto::metadata::{EntityDescription, RelationshipMetadata, RelationshipType};

#[derive(Debug, Clone)]
pub struct ContextualizerMetadata {
    metadata: ContextualizerEntityDescription,

    pub ignore_rules: ContextualizerMetadataIgnoreRules,

    // [Internal Specification for specific cases]
    pub internal_sepecification: Vec<ContextualizerInternalSpecification>,
}

#[derive(Debug, Clone)]
pub struct ContextualizerMetadataIgnoreRules {
    pub select: bool,
    pub filter: bool,
    pub expand: bool,
    pub compute: bool,
    pub orderby: bool,
    pub top: bool,
    pub skip: bool,
}

impl ContextualizerMetadata {
    pub fn new(metadata: EntityDescription) -> Self {
        let columns = metadata.columns.into_iter().map(|(k, v)| {
            (k, ContextualizerColumnMetadata::new( 
                v.column_name,
                v.column_type,
            ))
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

            internal_sepecification: Vec::new(),

            ignore_rules: ContextualizerMetadataIgnoreRules {
                select: false,
                filter: false,
                expand: false,
                compute: false,
                orderby: false,
                top: false,
                skip: false
            }
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
        (k.clone(), ContextualizerColumnMetadata::new(
            v.column_name.to_string(), 
            v.column_type
        ))
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
pub enum ContextualizerColumnMetadata {
    Original {
        column_name: String,
        column_type: TypeId,
    },
    Dynamic {
        column_name: String,
        column_type: TypeId,
        specification: ComputeSpecificationMetadata
    },
}

// [Compute Specification] //

#[derive(Debug, Clone)]
pub struct ComputeSpecificationMetadata {
    pub operation_type: ComputeOperation
}

#[derive(Debug, Clone)]
pub enum ComputeOperation {
    Arithmetic {
        operation: ComputeArithmeticOperation
    },
    Alias {
        operation: ComputeAliasOperation
    }
}

#[derive(Debug, Clone)]
pub struct ComputeOperand {
    pub table_name: String,
    pub column_name: String,
}

#[derive(Debug, Clone)]
pub enum ComputeArithmeticOperation {
    Addition {
        left_operand: ComputeOperand,
        right_operand: ComputeOperand
    },
    Subtration {
        left_operand: ComputeOperand,
        right_operand: ComputeOperand
    },
    Multiplication {
        left_operand: ComputeOperand,
        right_operand: ComputeOperand
    },
    Division {
        left_operand: ComputeOperand,
        right_operand: ComputeOperand
    },
    Module {
        left_operand: ComputeOperand,
        right_operand: ComputeOperand
    }
}

#[derive(Debug, Clone)]
pub struct ComputeAliasOperation {
    pub table_name: String,
    pub column_name: String
}

// [Compute Specification] //

impl ContextualizerColumnMetadata {
    pub fn new(column_name: String, column_type: TypeId) -> Self {
        ContextualizerColumnMetadata::Original {
            column_name,
            column_type,
        }
    }

    pub fn column_name(&self) -> &String {
        match self {
            ContextualizerColumnMetadata::Original { column_name, .. } => column_name,
            ContextualizerColumnMetadata::Dynamic { column_name, .. } => column_name,
        }
    }

    pub fn column_type(&self) -> TypeId {
        match self {
            ContextualizerColumnMetadata::Original { column_type, .. } => *column_type,
            ContextualizerColumnMetadata::Dynamic { column_type, .. } => *column_type,
        }
    }
}

// [Internal Specification] 
// [Specific situations like $count=true, that needs to set a property standalone in the response model to show the total count]

#[derive(Debug, Clone)]
pub enum ContextualizerInternalSpecification {
    Select {
        specification_type: ContextualizerInternalSelectSpecificationType,

    }
}

#[derive(Debug, Clone)]
pub enum ContextualizerInternalSelectSpecificationType {
    StandAlone {
        specification_attribute: String,
        column_name: String,
        column_type: TypeId,
    }
}

// [Internal Specification] //

impl Default for ContextualizerColumnMetadata {
    fn default() -> Self {
        ContextualizerColumnMetadata::Original {
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
    pub relationships: HashMap<String, ContextualizerRelationshipMetadata>
}
