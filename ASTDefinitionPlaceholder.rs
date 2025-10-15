// src/bluebirdc/ast.rs
//
// This file would define the data structures for the
// Abstract Syntax Tree (AST). The parser creates this tree.

// Example AST node for a function definition
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

// Example enum for all possible statements in the language
pub enum Statement {
    Let { name: String, value: Expression },
    // ... other statement types
}

pub enum Expression {
    LiteralInt(i64),
    // ... other expression types
}
