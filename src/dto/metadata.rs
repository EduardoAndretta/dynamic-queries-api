use std::collections::HashMap;
use std::any::TypeId;

#[derive(Debug, Clone)]
pub struct ColumnMetadata {
    pub column_name: String,
    pub column_type: TypeId,
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone)]
pub struct RelationshipMetadata {
    pub related_entity_metadata: EntityDescription,
    pub related_entity: String,
    pub navigation_property: String,
    pub relationship_type: RelationshipType,
    pub foreign_keys: Vec<String>,
    pub related_keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EntityDescription {
    pub table_name: String,
    pub columns: HashMap<String, ColumnMetadata>,
    pub relationships: HashMap<String, RelationshipMetadata>,
}

pub trait EntityMetadata {
    fn metadata() -> &'static EntityDescription;
}