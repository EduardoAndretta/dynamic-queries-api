use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::{metadata::EntityMetadata, query_params::QueryParams};

use crate::services::query::database::sqlite::operations::select::select_sqlite::SelectSqlite;
use crate::services::query::database::sqlite::operations::filter::filter_sqlite::FilterSqlite;
use crate::services::query::database::sqlite::operations::expand::expand_sqlite::ExpandSqlite;
use crate::services::query::database::sqlite::operations::orderby::orderby_sqlite::OrderbySqlite;
use crate::services::query::database::sqlite::operations::top::top_sqlite::TopSqlite;
use crate::services::query::database::sqlite::operations::skip::skip_sqlite::SkipSqlite;

pub struct SqliteQuery;

impl SqliteQuery {   
    pub fn build_query<T: EntityMetadata>(
        &self,
        options: &QueryParams,
        query_alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let mut sql: String = String::new();

        // SELECT clause 
        let sql_select = SelectSqlite::process::<T>(options.select.as_deref(), query_alias_manager)?;
        sql.push_str(&sql_select);
    
        // EXPAND (JOIN) clause 
        if let Some(expand) = &options.expand {
            let sql_join = ExpandSqlite::process::<T>(expand, query_alias_manager)?;
            sql.push_str(&sql_join);
        }
    
        // WHERE clause
        if let Some(filter) = &options.filter {
            let sql_filter = FilterSqlite::process::<T>(filter, query_alias_manager)?;
            sql.push_str(&sql_filter);
        }
    
        // ORDER BY clause
        if let Some(orderby) = &options.orderby {
            let sql_orderby = OrderbySqlite::process::<T>(orderby, query_alias_manager)?;
            sql.push_str(&sql_orderby);
        }
    
        // LIMIT and OFFSET
        if let Some(top) = &options.top {
            let sql_top = TopSqlite::process(top)?;
            sql.push_str(&sql_top);
        }
        if let Some(skip) = &options.skip {
            let sql_top = SkipSqlite::process(skip)?;
            sql.push_str(&sql_top);
        }
    
        println!("Final query: \n{}", sql);
        Ok(sql)
    }    
}