use std::collections::HashMap;

use crate::database::sqlite::common::context::contextualizer::{ContextualizerEntityDescription, ContextualizerColumnMetadata};

#[derive(Debug)]
pub struct DynamicStruct {
    pub fields: HashMap<String, PropertyType>,
}

impl DynamicStruct {
    pub fn new() -> Self {
        DynamicStruct {
            fields: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, key_path: &str, property: PropertyType) {
        let parts: Vec<&str> = key_path.split('.').collect();
        let mut current_fields = &mut self.fields;

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // Final part: insert the property
                match current_fields.entry(part.to_string()) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(property.clone());
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        let existing = entry.get();
                        match (existing, &property) {
                            // If the existing property is RelationalProperty, throw an error
                            (PropertyType::RelationalProperty { .. }, _) => {
                                panic!(
                                    "Key conflict: attempted to overwrite a RelationalProperty at `{}`",
                                    key_path
                                );
                            }
                            // Otherwise, overwrite safely
                            (_, _) => {
                                entry.insert(property.clone());
                            }
                        }
                    }
                }
            } else {
                // Intermediate part: ensure it's a RelationalProperty
                current_fields = match current_fields.entry(part.to_string()) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        let new_property = PropertyType::new_relational(ContextualizerEntityDescription {
                            table_name: "".to_string(),
                            columns: HashMap::new(),
                            relationships: HashMap::new(),
                        },
                        key_path.into());

                        let relational_property = entry.insert(new_property);
                        match relational_property {
                            PropertyType::RelationalProperty { ref mut properties, .. } => properties,
                            _ => unreachable!(),
                        }
                    }
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        match entry.into_mut() {
                            PropertyType::RelationalProperty { ref mut properties, .. } => properties,
                            _ => panic!(
                                "Key conflict: attempted to nest into a non-relational property at `{}`",
                                part
                            ),
                        }
                    }
                };
            }
        }
    }

    pub fn get_property(&self, key_path: &str) -> Option<&PropertyType> {
        let parts: Vec<&str> = key_path.split('.').collect();
        let mut current_fields = &self.fields;

        for (i, part) in parts.iter().enumerate() {
            match current_fields.get(*part) {
                Some(PropertyType::RelationalProperty { properties, .. }) => {
                    current_fields = properties;
                }
                Some(property) if i == parts.len() - 1 => {
                    return Some(property);
                }
                _ => return None,
            }
        }

        None
    }

}

#[derive(Debug, Clone)]
pub enum PropertyType {
    RelationalProperty {
        properties: HashMap<String, PropertyType>,
        metadata: ContextualizerEntityDescription,
        key: String
    },
    Property {
        metadata: ContextualizerColumnMetadata,
        key: String
    },
    UnknowProperty {
        name: String,
        key: String
    },
}

impl PropertyType {
    pub fn new_relational(metadata: ContextualizerEntityDescription, key: String) -> Self {
        PropertyType::RelationalProperty {
            properties: HashMap::new(),
            metadata,
            key
        }
    }

    pub fn new_property(metadata: ContextualizerColumnMetadata, key: String) -> Self {
        PropertyType::Property { metadata, key }
    }

    pub fn new_unknow_property(name: String, key: String) -> Self {
        PropertyType::UnknowProperty { name, key }
    }
}
