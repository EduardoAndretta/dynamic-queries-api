use std::collections::HashMap;

pub struct QueryAliasManager {
    table_alias_map: HashMap<String, String>,
    table_counter: usize,

    field_alias_map: HashMap<String, String>,
    field_counter: usize,

    internal_sepecification_standalone_field_counter: usize,
    internal_sepecification_standalone_field_alias_map: HashMap<String, String>,

    internal_sepecification_collective_field_counter: usize,
    internal_sepecification_collective_field_alias_map: HashMap<String, String>,
}

impl QueryAliasManager {
    pub fn new() -> Self {
        Self {
            table_alias_map: HashMap::new(),
            table_counter: 0,

            field_alias_map: HashMap::new(),
            field_counter: 0,

            internal_sepecification_standalone_field_alias_map: HashMap::new(),
            internal_sepecification_standalone_field_counter: 0,

            internal_sepecification_collective_field_alias_map: HashMap::new(),
            internal_sepecification_collective_field_counter: 0,
        }
    }

    pub fn get_or_create_table_alias(&mut self, table_name: &str) -> String {
        self.table_alias_map
            .entry(table_name.to_string())
            .or_insert_with(|| {
                self.table_counter += 1;
                format!("t{}", self.table_counter)
            })
            .clone()
    }

    pub fn get_or_create_field_alias(&mut self, field: &str) -> String {
        self.field_alias_map
            .entry(field.to_string())
            .or_insert_with(|| {
                self.field_counter += 1;
                format!("f{}", self.field_counter)
            })
            .clone()
    }

    pub fn get_or_create_internal_specification_collective_field_alias(&mut self, field: &str) -> String {
        self.internal_sepecification_collective_field_alias_map
            .entry(field.to_string())
            .or_insert_with(|| {
                self.internal_sepecification_collective_field_counter += 1;
                format!("is-c{}", self.internal_sepecification_collective_field_counter)
            })
            .clone()
    }

    pub fn get_or_create_internal_specification_standalone_field_alias(&mut self, field: &str) -> String {
        self.internal_sepecification_standalone_field_alias_map
            .entry(field.to_string())
            .or_insert_with(|| {
                self.internal_sepecification_standalone_field_counter += 1;
                format!("is-s{}", self.internal_sepecification_standalone_field_counter)
            })
            .clone()
    }

    pub fn get_field_alias(&self) -> &HashMap<String, String> {
        &self.field_alias_map
    }
}
