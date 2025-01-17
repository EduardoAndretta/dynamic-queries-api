use crate::services::query::common::alias_manager::QueryAliasManager;

use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;
use crate::services::query::database::sqlite::common::tokens::orderby::token::Token;

use super::token::tokenization::Tokenization;

pub struct Orderby;

impl Orderby {
    pub fn process(
        text: Option<&str>,
        alias_manager: &mut QueryAliasManager,
        contextualizer: &ContextualizerMetadata,
    ) -> Result<String, String> {

        let mut sql: String = String::from("");
        let mut tokens: Vec<Token> = Vec::new();

        if let Some(text) = text {
            tokens = Tokenization::tokenize(text, contextualizer)?;

            sql = Self::process_with_tokens(&tokens, alias_manager)?;
        }

        // [There no changes in context for while...]

        Ok(sql)
    }

    fn process_with_tokens(
        tokens: &[Token],
        alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        
        let mut sql: String = String::new();

        let mut properties = Vec::new();
           
        sql.push_str("ORDER BY\n");

        for token in tokens {
            match token {
                Token::Property { metadata, column_metadata } => {
                    let table_alias = alias_manager.get_or_create_table_alias(&metadata.table_name);
                    
                    properties.push(format!(
                        "[{}].[{}]",
                        table_alias, column_metadata.column_name
                    ));
                },
            }
        }

        sql.push_str(&properties.join(", "));

        sql.push_str("\n");

        Ok(sql.clone())
    }

}