use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::database::mssql::common::context::contextualizer::ContextualizerEntityDescription;
use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::database::sqlite::common::context::contextualizer::{ContextualizerColumnMetadata, ComputeArithmeticOperation, ComputeOperation, ComputeSpecificationMetadata};
use crate::services::query::database::sqlite::common::tokens::filter::token::Operation;

#[derive(Hash, Eq, PartialEq, Debug)]
enum OperationFilterType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    InValues,
}

lazy_static! {
    static ref OPERATORS: HashMap<OperationFilterType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(OperationFilterType::Equal, "=");
        map.insert(OperationFilterType::NotEqual, "<>");
        map.insert(OperationFilterType::GreaterThan, ">");
        map.insert(OperationFilterType::GreaterThanOrEqual, ">=");
        map.insert(OperationFilterType::LessThan, "<");
        map.insert(OperationFilterType::LessThanOrEqual, "<=");
        map.insert(OperationFilterType::Like, "LIKE");
        map.insert(OperationFilterType::InValues, "IN");
        map
    };
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum ArithmeticOperationType {
    Addition,
    Subtration,
    Multiplication,
    Division,
    Module
}

lazy_static! {
    static ref ARITHMETIC_OPERATORS: HashMap<ArithmeticOperationType, &'static str> = {
        let mut map = HashMap::new();
        map.insert(ArithmeticOperationType::Addition, "+");
        map.insert(ArithmeticOperationType::Subtration, "-");
        map.insert(ArithmeticOperationType::Multiplication, "*");
        map.insert(ArithmeticOperationType::Division, "/");
        map.insert(ArithmeticOperationType::Module, "%");
        map
    };
}

pub struct Compute;

impl Compute {
    pub fn process_computed_column(
        column_metadata: &ContextualizerColumnMetadata,
        filter_operation: &Operation,
        sql: &mut String,
        alias_manager: &mut QueryAliasManager,
    ) -> Result<(), String> {

        let get_operator = |operator_type: &OperationFilterType| -> &'static str {
            OPERATORS
                .get(operator_type)
                .ok_or_else(|| format!("Operator not found for {:?}", operator_type))
                .unwrap()
        };

        match column_metadata {
            ContextualizerColumnMetadata::Dynamic { specification, .. } => {
                match &specification.operation_type {
                    ComputeOperation::Alias { operation } => {

                        let original_table = alias_manager.get_or_create_table_alias(&operation.table_name);
                        let original_column_name = &operation.column_name;

                        match &filter_operation {
                            Operation::Equal { value } => {

                                let operator = get_operator(&OperationFilterType::Equal);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::NotEqual { value } => {

                                let operator = get_operator(&OperationFilterType::NotEqual);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::GreaterThan { value } => {

                                let operator = get_operator(&OperationFilterType::GreaterThan);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::GreaterThanOrEqual { value } => {

                                let operator = get_operator(&OperationFilterType::GreaterThanOrEqual);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::LessThan { value } => {

                                let operator = get_operator(&OperationFilterType::LessThan);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::LessThanOrEqual { value } => {

                                let operator = get_operator(&OperationFilterType::LessThanOrEqual);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::Like { value } => {

                                let operator = get_operator(&OperationFilterType::Like);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value
                                ));
                            },
                            Operation::InValues { value } => {

                                let operator = get_operator(&OperationFilterType::InValues);

                                sql.push_str(&format!(
                                    "([{}].[{}]) {} {}",
                                    original_table, original_column_name, operator, value.join(",")
                                ));
                            },
                        }
        
                    },
                    ComputeOperation::Arithmetic { operation } => {
                        let get_arithmetic_operator = |arithmetic_operator_type: &ArithmeticOperationType| -> &'static str {
                            ARITHMETIC_OPERATORS
                                .get(arithmetic_operator_type)
                                .ok_or_else(|| format!("Invalid $compute: Arithmetic operator not found for {:?}", arithmetic_operator_type))
                                .unwrap()
                        };
        
                        match operation {
                            ComputeArithmeticOperation::Addition { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Addition);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
              
                                match &filter_operation {
                                    Operation::Equal { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Equal);
          
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::NotEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Like);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::InValues);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} ({})",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value.join(",")
                                        ));
                                    },
                                }
                            },
                            ComputeArithmeticOperation::Subtration { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Subtration);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
              
                                match &filter_operation {
                                    Operation::Equal { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Equal);
          
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::NotEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Like);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::InValues);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} ({})",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value.join(",")
                                        ));
                                    },
                                }
                            },
                            ComputeArithmeticOperation::Multiplication { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Multiplication);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
              
                                match &filter_operation {
                                    Operation::Equal { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Equal);
          
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::NotEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Like);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::InValues);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} ({})",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value.join(",")
                                        ));
                                    },
                                }
                            },
                            ComputeArithmeticOperation::Division { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Division);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
              
                                match &filter_operation {
                                    Operation::Equal { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Equal);
          
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::NotEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Like);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::InValues);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} ({})",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value.join(",")
                                        ));
                                    },
                                }
                            }
                            ComputeArithmeticOperation::Module { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Module);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
              
                                match &filter_operation {
                                    Operation::Equal { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Equal);
          
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::NotEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::NotEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::GreaterThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::GreaterThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThan { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThan);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::LessThanOrEqual { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::LessThanOrEqual);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::Like { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::Like);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} {}",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value
                                        ));
                                    },
                                    Operation::InValues { value } => {
                                    
                                        let filter_operator = get_operator(&OperationFilterType::InValues);
                                    
                                        sql.push_str(&format!(
                                            "([{}].[{}] {} [{}].[{}]) {} ({})",
                                            left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, filter_operator, value.join(",")
                                        ));
                                    },
                                }
                            }
                        }
                    }
                }
            },
            _ => return Err(String::from("Provide only computed columns to 'process_computed_column'"))
        }
        Ok(())
    }
}