use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;

pub struct Skip;

impl Skip {   
    pub fn process(
        skip: Option<&i32>,
        contextualizer: &ContextualizerMetadata
    ) -> Result<String, String> {
        
        let error = |message: String| -> String {
            format!("Invalid $skip: {}", message)
        };

        let mut sql: String = String::new();

        if !contextualizer.ignore_rules.skip {
            if !contextualizer.ignore_rules.skip {
                if let Some(skip) = skip {
                    if skip.to_owned() < 0 {
                        return Err(error(String::from("Must be a non-negative integer")));
                    }
               
                    sql.push_str(&format!("OFFSET {}", skip));
                }
            }
        }
    
        Ok(sql)
    }
}