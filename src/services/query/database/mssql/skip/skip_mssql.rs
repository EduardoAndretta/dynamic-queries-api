use crate::dto::metadata::EntityMetadata;

pub struct SkipMssql;

impl SkipMssql {

    pub fn validate<T: EntityMetadata>(
        skip: &i32
    ) -> Result<(), String> {
        
        if skip.to_owned() < 0 {
            return Err("Invalid $skip: must be a non-negative integer".into());
        }
        
        Ok(())
    }
    
    pub fn process<T: EntityMetadata>(
        skip: &i32
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();

        sql.push_str(&format!("\nOFFSET {}", skip));
    
        Ok(sql)
    }
}