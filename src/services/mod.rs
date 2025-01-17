pub mod query {
    pub mod common {
        pub mod alias_manager;
    }

    pub mod database {

        pub mod sqlite {
            pub mod query;

            pub mod common {
                pub mod tokens {
                    pub mod filter {
                        pub mod token;
                    }
                    pub mod expand {
                        pub mod token;
                    }
                    pub mod select {
                        pub mod token;
                    }
                    pub mod orderby {
                        pub mod token;
                    }
                }
            }

            pub mod operations {

                pub mod expand {
                    pub mod expand;

                    pub mod token {
                        pub mod tokenization;
                    }
                    pub mod context {
                        pub mod context;
                    }
                }

                pub mod select {
                    pub mod select;

                    pub mod token {
                        pub mod tokenization;
                    }
                }
    
                pub mod filter {
                    pub mod filter;
    
                    pub mod token {
                        pub mod tokenization;
                    }
                }
   
                pub mod orderby {
                    pub mod orderby;

                    pub mod token {
                        pub mod tokenization;
                    }
                }
    
                pub mod top {
                    pub mod top;
                }
    
                pub mod skip {
                    pub mod skip;
                }
            } 
        }
    
        pub mod mssql {
            pub mod query;

            pub mod common {
                pub mod tokens {
                    pub mod filter {
                        pub mod token;
                    }
                    pub mod expand {
                        pub mod token;
                    }
                    pub mod select {
                        pub mod token;
                    }
                    pub mod orderby {
                        pub mod token;
                    }
                }
            }

            pub mod operations {

                pub mod expand {
                    pub mod expand;

                    pub mod token {
                        pub mod tokenization;
                    }
                    pub mod context {
                        pub mod context;
                    }
                }

                pub mod select {
                    pub mod select;

                    pub mod token {
                        pub mod tokenization;
                    }
                }
    
                pub mod filter {
                    pub mod filter;
    
                    pub mod token {
                        pub mod tokenization;
                    }
                }
   
                pub mod orderby {
                    pub mod orderby;

                    pub mod token {
                        pub mod tokenization;
                    }
                }
    
                pub mod top {
                    pub mod top;
                }
    
                pub mod skip {
                    pub mod skip;
                }
            } 
        }
    } 
} 
