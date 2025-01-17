use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::query_params::QueryParams;

use crate::database::sqlite::common::context::contextualizer::ContextualizerMetadata;

use crate::services::query::database::sqlite::operations::select::select::Select;
use crate::services::query::database::sqlite::operations::filter::filter::Filter;
use crate::services::query::database::sqlite::operations::expand::expand::Expand;
use crate::services::query::database::sqlite::operations::orderby::orderby::Orderby;
use crate::services::query::database::sqlite::operations::top::top::Top;
use crate::services::query::database::sqlite::operations::skip::skip::Skip;

pub struct Query;

impl Query {   
    pub fn build_query(
        &self,
        options: &QueryParams,
        query_alias_manager: &mut QueryAliasManager,
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<String, String> {

        // [$expand (With context changes)]
        let expand_text = Expand::process(options.expand.as_deref(), query_alias_manager, contextualizer)?;

        // [$select (Without context changes)]
        let select_text = Select::process(options.select.as_deref(), query_alias_manager, contextualizer)?;

        // [$filter (Without context changes)]
        let filter_text = Filter::process(options.filter.as_deref(), query_alias_manager, contextualizer)?;

        // [$orderby (Without context changes)]
        let orderby_text = Orderby::process(options.orderby.as_deref(), query_alias_manager, contextualizer)?;

        // [$top (Without context changes)]
        let mut top_text: String = String::from("");
        if let Some(text) = &options.top {
            top_text = Top::process(text)?;
        }

        // [$skip (Without context changes)]
        let mut skip_text: String = String::from("");
        if let Some(text) = &options.skip {
            skip_text = Skip::process(text)?;
        }

        let sql = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            select_text,
            expand_text,
            filter_text,
            orderby_text,
            top_text,
            skip_text
        );

        println!("Final query: \n{}", sql);
        
        Ok(sql)
    }    
}