use crate::database::mssql::common::context::contextualizer::ContextualizerMetadata;
use crate::services::query::database::mssql::common::tokens::compute::token::Token;

use super::{token::tokenization::Tokenization, context::context::Context};

pub struct Compute;

impl Compute {
    pub fn process(
        text: Option<&str>,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<(), String> {

        let error = |message: String| -> String {
            format!("Invalid $compute: {}", message)
        };

        let mut tokens: Vec<Token> = Vec::new();

        if let Some(text) = text {
            tokens = match Tokenization::tokenize(text, contextualizer) {
                Err(err) => return Err(error(err)),
                Ok(value) => value
            };
        }

        match Context::resolve_context(&tokens, contextualizer){
            Err(err) => return Err(error(err)),
            Ok(value) => value
        };

        Ok(())
    }   
}