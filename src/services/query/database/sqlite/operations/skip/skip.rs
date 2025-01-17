pub struct Skip;

impl Skip {   
    pub fn process(
        skip: &i32
    ) -> Result<String, String> {
        
        if skip.to_owned() < 0 {
            return Err("Invalid $skip: must be a non-negative integer".into());
        }

        let mut sql: String = String::new();

        sql.push_str(&format!("\nOFFSET {}", skip));
    
        Ok(sql)
    }
}