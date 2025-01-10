use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::dto::{metadata::EntityMetadata, query_params::QueryParams};

use crate::services::query::database::mssql::select::select_mssql::SelectMssql;
use crate::services::query::database::mssql::filter::filter_mssql::FilterMssql;
use crate::services::query::database::mssql::expand::expand_mssql::ExpandMssql;
use crate::services::query::database::mssql::orderby::orderby_mssql::OrderbyMssql;
use crate::services::query::database::mssql::top::top_mssql::TopMssql;
use crate::services::query::database::mssql::skip::skip_mssql::SkipMssql;

pub struct MssqlQuery;

impl MssqlQuery {
    pub fn validate_query<T: EntityMetadata>(
        &self, 
        options: &QueryParams
    ) -> Result<QueryParams, String> {
   
        if let Some(select) = &options.select {
            SelectMssql::validate::<T>(select)?;
        }
        if let Some(expand) = &options.expand {
            ExpandMssql::validate::<T>(expand)?;
        }

        if let Some(filter) = &options.filter {
            FilterMssql::validate::<T>(filter)?;
        }

        if let Some(orderby) = &options.orderby {
            OrderbyMssql::validate::<T>(orderby)?;
        }

        if let Some(top) = &options.top {
            TopMssql::validate::<T>(top)?;
        }

        if let Some(skip) = &options.skip {
            SkipMssql::validate::<T>(skip)?;
        }
    
        Ok(options.clone())
    }
    
    pub fn build_query<T: EntityMetadata>(
        &self,
        options: &QueryParams,
        query_alias_manager: &mut QueryAliasManager,
    ) -> Result<String, String> {
        let mut sql: String = String::new();

        // SELECT clause 
        let sql_select = SelectMssql::process::<T>(options.select.clone(), query_alias_manager)?;
        sql.push_str(&sql_select);
    
        // EXPAND (JOIN) clause 
        if let Some(expand) = &options.expand {
            let sql_join = ExpandMssql::process::<T>(expand, query_alias_manager)?;
            sql.push_str(&sql_join);
        }
    
        // WHERE clause
        if let Some(filter) = &options.filter {
            let sql_filter = FilterMssql::process::<T>(filter, query_alias_manager)?;
            sql.push_str(&sql_filter);
        }
    
        // ORDER BY clause
        if let Some(orderby) = &options.orderby {
            let sql_orderby = OrderbyMssql::process::<T>(orderby, query_alias_manager)?;
            sql.push_str(&sql_orderby);
        }
    
        // LIMIT and OFFSET
        if let Some(top) = &options.top {
            let sql_top = TopMssql::process::<T>(top)?;
            sql.push_str(&sql_top);
        }
        if let Some(skip) = &options.skip {
            let sql_top = SkipMssql::process::<T>(skip)?;
            sql.push_str(&sql_top);
        }
    
        println!("Final query: \n{}", sql);
        Ok(sql)
    }    
}