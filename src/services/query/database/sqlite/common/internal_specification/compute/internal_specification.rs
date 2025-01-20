use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;

pub struct InternalSpecification;

impl InternalSpecification {
    pub fn process_internal_specification(contextualizer: &ContextualizerMetadata) -> Result<(), String> {

        let internal_specifications = &contextualizer.internal_sepecifications;

        let metadata = &contextualizer.get_context();

        // [Collective]
        for internal_specification in &internal_specifications.collective {
            if metadata.columns.contains_key(&internal_specification.column_name) {
                return Err(format!("The computed alias '{}' is reserved for an collective internal specification field with the same name.", &internal_specification.column_name))
            }
        }

        Ok(())
    }
}