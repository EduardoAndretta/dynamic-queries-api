use crate::{database::sqlite::common::context::contextualizer::ContextualizerMetadata, services::query::common::alias_manager::QueryAliasManager};

pub struct InternalSpecification;

impl InternalSpecification {
    pub fn process_internal_specification(
        properties: &mut Vec<String>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<(), String> {

        let internal_specifications = &contextualizer.internal_sepecifications;

        // [Collective]
        for internal_specification in &internal_specifications.collective {
            let internal_specification_field_alias = alias_manager.get_or_create_internal_specification_collective_field_alias(&internal_specification.column_name);

            properties.push(format!(
                "{} AS [{}]", 
                internal_specification.specification_attribute, internal_specification_field_alias
            ));
        }

        // [Standalone]
        for internal_specification in &internal_specifications.standalone {
            let internal_specification_field_alias = alias_manager.get_or_create_internal_specification_standalone_field_alias(&internal_specification.column_name);

            properties.push(format!(
                "{} AS [{}]", 
                internal_specification.specification_attribute, internal_specification_field_alias
            ));
        }

        Ok(())
    }
}