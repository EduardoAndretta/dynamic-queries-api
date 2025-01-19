use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::services::query::common::alias_manager::QueryAliasManager;
use crate::database::sqlite::common::context::contextualizer::{ContextualizerColumnMetadata, ComputeArithmeticOperation, ComputeOperation};

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
        properties: &mut Vec<String>,
        alias_manager: &mut QueryAliasManager,
    ) -> Result<(), String> {

        match column_metadata {
            ContextualizerColumnMetadata::Dynamic { column_name, specification, .. } => {
                match &specification.operation_type {
                    ComputeOperation::Alias { operation } => {
        
                        let original_table = alias_manager.get_or_create_table_alias(&operation.table_name);
                        let original_column_name = &operation.column_name;
        
                        let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                        properties.push(format!(
                            "[{}].[{}] AS [{}]", 
                            original_table, original_column_name, alias_column_name
                        ));
        
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
        
                                let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                                properties.push(format!(
                                    "([{}].[{}] {} [{}].[{}]) AS [{}]", 
                                    left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, alias_column_name
                                ));
                            },
                            ComputeArithmeticOperation::Subtration { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Subtration);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
        
                                let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                                properties.push(format!(
                                    "([{}].[{}] {} [{}].[{}]) AS [{}]", 
                                    left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, alias_column_name
                                ));
                            },
                            ComputeArithmeticOperation::Multiplication { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Multiplication);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
        
                                let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                                properties.push(format!(
                                    "([{}].[{}] {} [{}].[{}]) AS [{}]", 
                                    left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, alias_column_name
                                ));
                            },
                            ComputeArithmeticOperation::Division { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Division);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
        
                                let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                                properties.push(format!(
                                    "([{}].[{}] {} [{}].[{}]) AS [{}]", 
                                    left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, alias_column_name
                                ));
                            },
                            ComputeArithmeticOperation::Module { left_operand, right_operand} => {
                                
                                let operator = get_arithmetic_operator(&ArithmeticOperationType::Module);
        
                                let left_operand_table = alias_manager.get_or_create_table_alias(&left_operand.table_name);
                                let left_operand_column_name = &left_operand.column_name;
        
                                let right_operand_table = alias_manager.get_or_create_table_alias(&right_operand.table_name);
                                let right_operand_column_name = &right_operand.column_name;
        
                                let alias_column_name = alias_manager.get_or_create_field_alias(&column_name);
        
                                properties.push(format!(
                                    "([{}].[{}] {} [{}].[{}]) AS [{}]", 
                                    left_operand_table, left_operand_column_name, operator, right_operand_table, right_operand_column_name, alias_column_name
                                ));
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