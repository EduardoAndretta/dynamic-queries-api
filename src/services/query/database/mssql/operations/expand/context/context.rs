use crate::database::mssql::common::context::contextualizer::{ContextualizerMetadata, ContextualizerEntityDescription, ContextualizerRelationshipMetadata};
use crate::services::query::database::mssql::common::tokens::expand::token::Token;

pub struct Context;

impl Context {
    pub fn resolve_context(
        tokens: &[Token],
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<(), String> {  

        let mut metadata = contextualizer.get_context();
         
        for token in tokens {
            match token {
                Token::Property { metadata_dependencies } => {
                    
                    let mut sorted_metadata_dependencies: Vec<(&i32, &(ContextualizerEntityDescription, ContextualizerRelationshipMetadata))> =
                        metadata_dependencies.iter().collect();

                    sorted_metadata_dependencies.sort_by_key(|&(index, _)| index);
    
                    let valid_dependency: String = sorted_metadata_dependencies
                        .into_iter()
                        .map(|(_, (_, relationship_metadata))| relationship_metadata.related_entity_metadata.table_name.clone())
                        .collect::<Vec<String>>()
                        .join("/");

                    Self::resolve_context_metadata(&mut metadata, &valid_dependency)?;
                }
            }
        }
        contextualizer.update_context(metadata.clone());
    
        Ok(())
    }

    fn resolve_context_nested_metadata(
        metadata: &mut ContextualizerEntityDescription,
        dependency: &str,
    ) -> Result<(), String> {
        let parts: Vec<&str> = dependency.split('/').collect();
    
        if parts.len() < 2 {
            return Err(format!("Invalid nested token format: {}", dependency));
        }
    
        let mut current_metadata = metadata;
    
        for part in parts {
            let relationship_metadata = current_metadata
                .relationships
                .get_mut(part)
                .ok_or_else(|| format!("Invalid nested related entity: {}", part))?;
    
            relationship_metadata.expanded = true;
    
            current_metadata = &mut relationship_metadata.related_entity_metadata;
        }
    
        Ok(())
    }

    fn resolve_context_non_nested_metadata(metadata: &mut ContextualizerEntityDescription, dependency: &str) -> Result<(), String> {
        
        let relationship_metadata = metadata
                .relationships
                .get_mut(dependency)
                .ok_or_else(|| format!("Invalid related entity: {}", dependency))?;
    
        relationship_metadata.expanded = true;
    
        return Ok(());
    }

    fn resolve_context_metadata(metadata: &mut ContextualizerEntityDescription, dependency: &str) -> Result<(), String> {
        if dependency.contains('/') {
            Self::resolve_context_nested_metadata(metadata, dependency)
        } else {
            Self::resolve_context_non_nested_metadata(metadata, dependency)
        }
    }
}