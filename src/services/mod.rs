pub mod query {
    pub mod common {
        pub mod alias_manager;
    }

    pub mod database {
        pub mod sqlite {
            pub mod sqlite_query;

            pub mod select {
                pub mod select_sqlite;
            }

            pub mod filter {
                pub mod filter_sqlite;
            }

            pub mod expand {
                pub mod expand_sqlite;
            }

            pub mod orderby {
                pub mod orderby_sqlite;
            }

            pub mod top {
                pub mod top_sqlite;
            }

            pub mod skip {
                pub mod skip_sqlite;
            }
        }
    
        pub mod mssql {
            pub mod mssql_query;

            pub mod select {
                pub mod select_mssql;
            }

            pub mod filter {
                pub mod filter_mssql;
            }

            pub mod expand {
                pub mod expand_mssql;
            }

            pub mod orderby {
                pub mod orderby_mssql;
            }

            pub mod top {
                pub mod top_mssql;
            }

            pub mod skip {
                pub mod skip_mssql;
            }
        }
    } 
} 
