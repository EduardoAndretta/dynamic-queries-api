use std::any::TypeId;

use crate::database::mssql::common::context::contextualizer::{ContextualizerColumnMetadata, ContextualizerEntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Operation {
        operation_type: Operation
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Arithmetic {
        operation: ArithmeticOperation
    },
    Alias {
        operation: AliasOperation
    }
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub metadata: ContextualizerEntityDescription,
    pub column_metadata: ContextualizerColumnMetadata,
}

#[derive(Debug, Clone)]
pub enum ArithmeticOperation {
    Addition {
        left_operand: Operand,
        right_operand: Operand,
        alias_name: String,
        alias_type: TypeId
    },
    Subtration {
        left_operand: Operand,
        right_operand: Operand,
        alias_name: String,
        alias_type: TypeId
    },
    Multiplication {
        left_operand: Operand,
        right_operand: Operand,
        alias_name: String,
        alias_type: TypeId
    },
    Division {
        left_operand: Operand,
        right_operand: Operand,
        alias_name: String,
        alias_type: TypeId
    },
    Module {
        left_operand: Operand,
        right_operand: Operand,
        alias_name: String,
        alias_type: TypeId
    }
}

#[derive(Debug, Clone)]
pub struct AliasOperation {
    pub metadata: ContextualizerEntityDescription,
    pub column_metadata: ContextualizerColumnMetadata,
    pub alias_name: String,
    pub alias_type: TypeId
}


impl Token {
    pub fn new_operation(operation_type: Operation) -> Self {
        Token::Operation {
            operation_type
        }
    }
}
