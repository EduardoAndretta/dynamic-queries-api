use std::any::TypeId;

use crate::database::mssql::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerMetadata};

pub struct Context;

impl Context {
    pub fn resolve_context(
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<(), String> {  

        let mut metadata = contextualizer.get_context();

        let column_name = "count".to_string();
        let column_type = TypeId::of::<i64>();

        metadata.columns.insert(column_name.clone(), ContextualizerColumnMetadata::new(column_name, column_type));

        contextualizer.update_context(metadata.clone());
    
        Ok(())
    }
}