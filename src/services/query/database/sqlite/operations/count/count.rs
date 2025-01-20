use std::any::TypeId;

use crate::database::sqlite::common::context::contextualizer::{ContextualizerMetadata, ContextualizerMetadataFlow, StandaloneInternalSpecification};

pub struct Count;

impl Count {
    pub fn process(
        count: Option<&bool>,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<(), String> {

        match contextualizer.flow {
            ContextualizerMetadataFlow::Count => {

                // [count internal-specification]
                contextualizer.internal_sepecifications.standalone.insert(
                    StandaloneInternalSpecification {
                        specification_attribute: "COUNT(*) OVER ()".to_string(), 
                        column_name: "count".to_string(), 
                        column_type: TypeId::of::<i64>() ,
                        default_value: "0".to_string()
                });
            }
            ContextualizerMetadataFlow::Ordinary => {

                if !contextualizer.ignore_rules.count {
                    if let Some(count) = count {
                        if *count {
                            // [count internal-specification]
                            contextualizer.internal_sepecifications.standalone.insert(
                                StandaloneInternalSpecification {
                                    specification_attribute: "COUNT(*) OVER ()".to_string(), 
                                    column_name: "count".to_string(), 
                                    column_type: TypeId::of::<i64>() ,
                                    default_value: "0".to_string()
                            });
                        }
                    }   
                }
            }
        }

        Ok(())
    }   
}