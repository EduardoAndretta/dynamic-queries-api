use std::collections::HashSet;

use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;

pub struct InternalSpecification;

impl InternalSpecification {
    pub fn process_internal_specification(contextualizer: &ContextualizerMetadata) -> Result<(), String> {

        let internal_specifications = &contextualizer.internal_sepecifications;

        let metadata = &contextualizer.get_context();

        let mut previous_properties: HashSet<String> = HashSet::new();

        // [Collective]
        for internal_specification in &internal_specifications.collective {
            if previous_properties.contains(&internal_specification.column_name) {
                return Err(String::from(format!("Collective internal-specification '{}' already mapped", &internal_specification.column_name)));
            }
            previous_properties.insert(internal_specification.column_name.to_string());

            if metadata.columns.contains_key(&internal_specification.column_name) {
                return Err(format!("The column_name '{}' is reserved for an collective internal specification field with the same name.", &internal_specification.column_name))
            }
        }

        previous_properties.clear();

        // [Standalone]
        for internal_specification in &internal_specifications.standalone {
            if previous_properties.contains(&internal_specification.column_name) {
                return Err(String::from(format!("Standalone internal-specification '{}' already mapped", &internal_specification.column_name)));
            }
            previous_properties.insert(internal_specification.column_name.to_string());
        }

        Ok(())
    }
}