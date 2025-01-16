pub struct TopSqlite;

impl TopSqlite {
    pub fn process(
        top: &i32
    ) -> Result<String, String> {

        if top.to_owned() < 0 {
            return Err("Invalid $top: must be a non-negative integer".into());
        }

        let mut sql: String = String::new();

        sql.push_str(&format!("\nLIMIT {}", top));
    
        Ok(sql)
    }
}