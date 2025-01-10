use std::collections::HashMap;

pub struct QueryAliasManager {
    table_alias_map: HashMap<String, String>,
    field_alias_map: HashMap<String, String>,
    table_counter: usize,
    field_counter: usize,
}

impl QueryAliasManager {
    pub fn new() -> Self {
        Self {
            table_alias_map: HashMap::new(),
            field_alias_map: HashMap::new(),
            table_counter: 0,
            field_counter: 0,
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

    pub fn get_or_create_field_alias(&mut self, nested_path: &str) -> String {
        self.field_alias_map
            .entry(nested_path.to_string())
            .or_insert_with(|| {
                self.field_counter += 1;
                format!("f{}", self.field_counter)
            })
            .clone()
    }

    pub fn get_field_alias(&self) -> &HashMap<String, String> {
        &self.field_alias_map
    }
}
