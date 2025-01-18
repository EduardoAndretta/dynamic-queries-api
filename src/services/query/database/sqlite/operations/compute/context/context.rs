use crate::database::sqlite::common::context::contextualizer::{ComputeAliasOperation, ComputeArithmeticOperation, ComputeOperand, ComputeOperation, ComputeSpecificationMetadata, ContextualizerColumnMetadata, ContextualizerMetadata};
use crate::services::query::database::sqlite::common::tokens::compute::token::{ArithmeticOperation, Operation, Token};

pub struct Context;

impl Context {
    pub fn resolve_context(
        tokens: &[Token],
        contextualizer: &mut ContextualizerMetadata,
    ) -> Result<(), String> {  

        let mut metadata = contextualizer.get_context();

        for token in tokens {
            match token {
                Token::Operation { operation_type: operation} => {
                    match operation {
                        Operation::Alias { operation} => {

                            metadata.columns.insert(operation.alias_name.to_string(), 
                                ContextualizerColumnMetadata::Dynamic {

                                    column_name: operation.alias_name.to_string(),
                                    column_type: operation.alias_type,

                                    specification: ComputeSpecificationMetadata {
                                        operation_type: ComputeOperation::Alias {
                                            operation: ComputeAliasOperation {
                                                table_name: operation.metadata.table_name.clone(),
                                                column_name: operation.column_metadata.column_name().clone()
                                            }
                                        }
                                    }
                                });

                        },
                        Operation::Arithmetic { operation } => {

                            match operation {
                                ArithmeticOperation::Addition { left_operand, right_operand, alias_name, alias_type } => {
                                    metadata.columns.insert(alias_name.to_string(), 
                                        ContextualizerColumnMetadata::Dynamic {
                                        
                                            column_name: alias_name.to_string(),
                                            column_type: *alias_type,
                                        
                                            specification: ComputeSpecificationMetadata {
                                                operation_type: ComputeOperation::Arithmetic {
                                                    operation: ComputeArithmeticOperation::Addition {
                                                        left_operand: ComputeOperand {
                                                            table_name: left_operand.metadata.table_name.clone(),
                                                            column_name: left_operand.column_metadata.column_name().clone()
                                                        },
                                                        right_operand: ComputeOperand {
                                                            table_name: right_operand.metadata.table_name.clone(),
                                                            column_name: right_operand.column_metadata.column_name().clone()
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                },
                                ArithmeticOperation::Subtration { left_operand, right_operand, alias_name, alias_type } => {
                                    metadata.columns.insert(alias_name.to_string(), 
                                        ContextualizerColumnMetadata::Dynamic {
                                        
                                            column_name: alias_name.to_string(),
                                            column_type: *alias_type,
                                        
                                            specification: ComputeSpecificationMetadata {
                                                operation_type: ComputeOperation::Arithmetic {
                                                    operation: ComputeArithmeticOperation::Subtration {
                                                        left_operand: ComputeOperand {
                                                            table_name: left_operand.metadata.table_name.clone(),
                                                            column_name: left_operand.column_metadata.column_name().clone()
                                                        },
                                                        right_operand: ComputeOperand {
                                                            table_name: right_operand.metadata.table_name.clone(),
                                                            column_name: right_operand.column_metadata.column_name().clone()
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                },
                                ArithmeticOperation::Multiplication { left_operand, right_operand, alias_name, alias_type } => {
                                    metadata.columns.insert(alias_name.to_string(), 
                                        ContextualizerColumnMetadata::Dynamic {
                                        
                                            column_name: alias_name.to_string(),
                                            column_type: *alias_type,
                                        
                                            specification: ComputeSpecificationMetadata {
                                                operation_type: ComputeOperation::Arithmetic {
                                                    operation: ComputeArithmeticOperation::Multiplication {
                                                        left_operand: ComputeOperand {
                                                            table_name: left_operand.metadata.table_name.clone(),
                                                            column_name: left_operand.column_metadata.column_name().clone()
                                                        },
                                                        right_operand: ComputeOperand {
                                                            table_name: right_operand.metadata.table_name.clone(),
                                                            column_name: right_operand.column_metadata.column_name().clone()
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                },
                                ArithmeticOperation::Division { left_operand, right_operand, alias_name, alias_type } => {
                                    metadata.columns.insert(alias_name.to_string(), 
                                        ContextualizerColumnMetadata::Dynamic {
                                        
                                            column_name: alias_name.to_string(),
                                            column_type: *alias_type,
                                        
                                            specification: ComputeSpecificationMetadata {
                                                operation_type: ComputeOperation::Arithmetic {
                                                    operation: ComputeArithmeticOperation::Division {
                                                        left_operand: ComputeOperand {
                                                            table_name: left_operand.metadata.table_name.clone(),
                                                            column_name: left_operand.column_metadata.column_name().clone()
                                                        },
                                                        right_operand: ComputeOperand {
                                                            table_name: right_operand.metadata.table_name.clone(),
                                                            column_name: right_operand.column_metadata.column_name().clone()
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                },
                                ArithmeticOperation::Module { left_operand, right_operand, alias_name, alias_type } => {
                                    metadata.columns.insert(alias_name.to_string(), 
                                        ContextualizerColumnMetadata::Dynamic {
                                        
                                            column_name: alias_name.to_string(),
                                            column_type: *alias_type,
                                        
                                            specification: ComputeSpecificationMetadata {
                                                operation_type: ComputeOperation::Arithmetic {
                                                    operation: ComputeArithmeticOperation::Module {
                                                        left_operand: ComputeOperand {
                                                            table_name: left_operand.metadata.table_name.clone(),
                                                            column_name: left_operand.column_metadata.column_name().clone()
                                                        },
                                                        right_operand: ComputeOperand {
                                                            table_name: right_operand.metadata.table_name.clone(),
                                                            column_name: right_operand.column_metadata.column_name().clone()
                                                        }
                                                    }
                                                }
                                            }
                                        });
                                }
                            }

                        }
                    }

                }
            }
        }

        contextualizer.update_context(metadata.clone());

        Ok(())
    }
}