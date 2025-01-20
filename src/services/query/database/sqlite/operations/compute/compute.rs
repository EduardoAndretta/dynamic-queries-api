use crate::{database::sqlite::common::context::contextualizer::ContextualizerMetadata, services::query::database::sqlite::common::internal_specification::compute::internal_specification::InternalSpecification};

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

        if !contextualizer.ignore_rules.compute {
            if let Some(text) = text {
                let tokens = match Tokenization::tokenize(text, contextualizer) {
                    Err(err) => return Err(error(err)),
                    Ok(value) => value
                };

                match Context::resolve_context(&tokens, contextualizer){
                    Err(err) => return Err(error(err)),
                    Ok(value) => value
                };
            }   
        }

        // [internal-specification]
        match InternalSpecification::process_internal_specification(contextualizer) {
            Err(err) => return Err(error(err)),
            Ok(value) => value
        };

        Ok(())
    }   
}