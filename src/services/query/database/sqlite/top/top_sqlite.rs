use crate::dto::metadata::EntityMetadata;

pub struct TopSqlite;

impl TopSqlite {

    pub fn validate<T: EntityMetadata>(
        top: &i32
    ) -> Result<(), String> {
        
        if top.to_owned() < 0 {
            return Err("Invalid $top: must be a non-negative integer".into());
        }

        Ok(())
    }
    
    pub fn process<T: EntityMetadata>(
        top: &i32
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();

        sql.push_str(&format!("\nLIMIT {}", top));
    
        Ok(sql)
    }
}