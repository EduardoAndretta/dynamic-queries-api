use std::collections::HashSet;

use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::mssql::common::context::contextualizer::{ContextualizerEntityDescription, ContextualizerMetadata, ContextualizerRelationshipMetadata, ContextualizerRelationshipType};
use crate::services::query::database::mssql::common::tokens::expand::token::Token;

use super::{token::tokenization::Tokenization, context::context::Context};

pub struct Expand;

impl Expand {
    pub fn process(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::from("");
        let mut tokens: Vec<Token> = Vec::new();

        if let Some(text) = text {
            tokens = Tokenization::tokenize(text, contextualizer)?;

            sql = Self::process_with_tokens(&tokens, alias_manager)?;
        }

        Context::resolve_context(&tokens, contextualizer)?;

        Ok(sql)
    }
 
    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();

        let mut previous_properties: HashSet<(String, String)> = HashSet::new();
           
        let mut is_first_join = true;

        for token in tokens {
            match token {
                Token::Property { metadata_dependencies } => {
                    let mut sorted_metadata_dependencies: Vec<(&i32, &(ContextualizerEntityDescription, ContextualizerRelationshipMetadata))> =
                    metadata_dependencies.iter().collect();
                    
                    sorted_metadata_dependencies.sort_by_key(|&(index, _)| index);
                
                    for (_, (metadata, relationship_metadata)) in sorted_metadata_dependencies {
                        let property_key = (relationship_metadata.related_entity_metadata.table_name.clone(), metadata.table_name.clone());
                        
                        // [Verify if the table was already joined with this current combination]
                        if previous_properties.contains(&property_key) {
                            continue;
                        }

                        previous_properties.insert(property_key);

                        let foreign_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
                        let related_alias = alias_manager.get_or_create_table_alias(&relationship_metadata.related_entity_metadata.table_name);
                
                        let join_conditions: Vec<String> = relationship_metadata
                            .foreign_keys
                            .iter()
                            .zip(relationship_metadata.related_keys.iter())
                            .map(|(foreign_key, related_key)| format!("[{}].[{}] = [{}].[{}]", foreign_alias, foreign_key, related_alias, related_key))
                            .collect();
                
                        if join_conditions.is_empty() {
                            return Err(format!(
                                "No foreign key mappings found for relationship: {}",
                                metadata.table_name
                            ));
                        }
                
                        // [Only add a newline if this is not the first join]
                        if !is_first_join {
                            sql.push_str("\n");
                        } else {
                            is_first_join = false;
                        }

                        let join_type = match relationship_metadata.relationship_type {
                            ContextualizerRelationshipType::OneToOne => "INNER JOIN",
                            ContextualizerRelationshipType::OneToMany => "LEFT JOIN",
                            ContextualizerRelationshipType::ManyToOne => "LEFT JOIN",
                            ContextualizerRelationshipType::ManyToMany => "LEFT JOIN"
                        };

                        sql.push_str(&format!(
                            "{} [{}] AS [{}] ON {}",
                            join_type,
                            relationship_metadata.related_entity_metadata.table_name,
                            related_alias,
                            join_conditions.join(" AND ")
                        ));
                    }
                },
            }
        }
        Ok(sql.clone())
    }
}