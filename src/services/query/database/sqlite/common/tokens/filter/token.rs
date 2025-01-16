use crate::dto::metadata::{ColumnMetadata, EntityDescription};

#[derive(Debug, Clone)]
pub enum Token {
    Property {
        metadata: EntityDescription,
        column_metadata: ColumnMetadata,
        operation: Operation,
    },
    Increment {
        increment: Increment
    },
    Parentheses {
        parentheses: Parentheses
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Equal {
        value: String
    },
    NotEqual {
        value: String
    },
    GreaterThan {
        value: String
    },
    GreaterThanOrEqual {
        value: String
    },
    LessThan {
        value: String
    },
    LessThanOrEqual {
        value: String
    },
    Like {
        value: String
    },
    InValues {
        value: Vec<String>
    }
}

impl Operation {
    pub fn equal(value: String) -> Self {
        Operation::Equal { value }
    }
    pub fn not_equal(value: String) -> Self {
        Operation::NotEqual { value }
    }
    pub fn greater_than(value: String) -> Self {
        Operation::GreaterThan { value }
    }
    pub fn greater_than_or_equal(value: String) -> Self {
        Operation::GreaterThanOrEqual { value }
    }
    pub fn less_than(value: String) -> Self {
        Operation::LessThan { value }
    }
    pub fn less_than_or_equal(value: String) -> Self {
        Operation::LessThanOrEqual { value }
    }
    pub fn like(value: String) -> Self {
        Operation::Like { value: value }
    }
    pub fn in_values(value: Vec<String>) -> Self {
        Operation::InValues { value: value }
    }
}

#[derive(Debug, Clone)]
pub enum Increment {
    And,
    Or
}

#[derive(Debug, Clone)]
pub enum Parentheses {
    Begin,
    End
}

impl Token {
    pub fn new_property(metadata: EntityDescription, column_metadata: ColumnMetadata, operation: Operation) -> Self {
        Token::Property {
            metadata,
            column_metadata,
            operation
        }
    }
    pub fn new_increment(increment: Increment) -> Self {
        Token::Increment { increment }
    }

    pub fn new_parentheses(parentheses: Parentheses) -> Self {
        Token::Parentheses { parentheses }
    }
}
