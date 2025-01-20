use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;

pub struct Top;

impl Top {   
    pub fn process(
        top: Option<&i32>,
        contextualizer: &ContextualizerMetadata
    ) -> Result<String, String> {
        
        let error = |message: String| -> String {
            format!("Invalid $top: {}", message)
        };

        let mut sql: String = String::new();

        if !contextualizer.ignore_rules.top {
            if let Some(top) = top {
                if top.to_owned() < 0 {
                    return Err(error(String::from("Must be a non-negative integer")));
                }
           
                sql.push_str(&format!("LIMIT {}", top));
            }
        }
    
        Ok(sql)
    }
}