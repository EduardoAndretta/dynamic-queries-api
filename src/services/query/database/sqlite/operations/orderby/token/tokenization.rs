use std::{cell::RefCell, collections::HashSet, rc::Rc};
use dynamic_queries_api::EventfulPeekable;

use crate::{dto::metadata::{ColumnMetadata, EntityDescription, EntityMetadata}, services::query::database::sqlite::common::tokens::orderby::token::Token};

pub struct Tokenization;

impl Tokenization {
    fn split(text: &str) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        
        for char in text.chars() {
            match char {
                ',' => {
                    if !current_token.trim().is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(char.to_string());
                },
                _ => {
                    current_token.push(char);
                }
            }
        }
    
        // [Add the last token if necessary]
        if !current_token.trim().is_empty() {
            tokens.push(current_token.trim().to_string());
        }
    
        Ok(tokens)
    }

    fn resolve_nested_metadata(metadata: &EntityDescription, token: &str) -> Result<(EntityDescription, ColumnMetadata), String> {
        let parts: Vec<&str> = token.split('/').collect();

        if parts.len() < 2 {
            return Err(format!("Invalid nested token format: {}", token));
        }

        let mut current_metadata = metadata.clone();

        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                
                // [Last part should be a field in the related entity]
                let metadata = current_metadata.clone();

                let column_metadata = metadata
                    .columns
                    .get(*part)
                    .cloned()
                    .ok_or_else(|| format!("Invalid token in related entity: {}", part))?;

                return Ok((metadata, column_metadata));

            } else {
                // [Intermediate parts should be relationships]
                let relationship = current_metadata
                    .relationships
                    .get(*part)
                    .ok_or_else(|| format!("Invalid related entity: {}", part))?;
                current_metadata = relationship.related_entity_metadata.clone();
            }
        }

        Err(format!("Unexpected error resolving metadata for token: {}", token))
    }

    fn resolve_non_nested_metadata(metadata: &EntityDescription, token: &str) -> Result<(EntityDescription, ColumnMetadata), String> {
        
        let metadata = metadata.clone();

        let column_metadata = metadata
            .columns
            .get(token)
            .cloned()
            .ok_or_else(|| format!("Invalid token: {}", token))?;

            return Ok((metadata, column_metadata));
    }

    fn get_metadata(metadata: &EntityDescription, token: &str) -> Result<(EntityDescription, ColumnMetadata), String> {
        if token.contains('/') {
            Self::resolve_nested_metadata(metadata, token)
        } else {
            Self::resolve_non_nested_metadata(metadata, token)
        }
    }

    pub fn tokenize<T: EntityMetadata>(text: &str) -> Result<Vec<Token>, String> {
        let metadata = T::metadata();
    
        let mut tokens: Vec<Token> = Vec::new();

        let mut previous_properties: HashSet<String> = HashSet::new();
    
        let splited: &[String] = &Self::split(text)?;

        // [Context]
        let context: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
            
        let mut tokens_iter = EventfulPeekable::new(
            splited.iter(),
            {
                let context = Rc::clone(&context);
                move |item| {
                    context.borrow_mut().push_str(&format!(" {} ", item));
                }
            },
        );

        // [Parser state]: Tracks the last token type
        enum LastToken {
            None,
            Comma,
            Property
        }
    
        let mut last_token = LastToken::None;

        let error_with_context = |message: String| -> String {
            format!("Invalid $orderby: {} [Context: {}]", message, context.borrow())
        };
    
        while let Some(token) = tokens_iter.next() {
            match token.as_str() {
                "," => {
                    match last_token {
                        LastToken::Property => {
                            last_token = LastToken::Comma;
                        },
                        _ => {
                            return Err(error_with_context(String::from("Comma must follow a property")));
                        }
                    }
                },
                _ => {
                    match last_token {
                        LastToken::None | LastToken::Comma => {

                            if previous_properties.contains(token) {
                                return Err(error_with_context( String::from(format!("Property '{}' already mapped", token))));
                            }
                            previous_properties.insert(token.to_string());

                            // [Current Metadata]
                            let (metadata, column_metadata ) = Self::get_metadata(&metadata, token)
                            .map_err(|err| error_with_context(err))?;

                            tokens.push(Token::new_property(metadata, column_metadata));
                            
                            last_token = LastToken::Property;
                        },
                        _ => {
                            return Err(error_with_context( String::from("Property must follow a comma or be setted in begin")));
                        }
                    }
                }
            }
        }

        match last_token {
            LastToken::Comma => {
                return Err(error_with_context(String::from("Ends unexpectedly; incomplete structure.")));
            }
            _ => Ok(tokens),
        }
    }
}