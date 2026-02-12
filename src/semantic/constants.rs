use crate::prelude::*;
pub use thiserror::Error;

pub type VarDict = HashMap<Scope, HashMap<String, Variable>>;
pub type FuncDict = HashMap<Scope, FuncDetails>;

#[derive(Debug)]
pub struct FuncDetails {
    pub returntype: Option<DataType>,
    pub parameters: Vec<DataType>,
}

impl FuncDetails {
    pub fn new() -> Self {
        Self {
            returntype: None,
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub datatype: DataType,
    pub decl_line: usize,
    pub uninit: bool,
}

impl Variable {
    pub fn new(datatype: DataType, decl_line: usize) -> Self {
        Self {
            datatype,
            decl_line,
            uninit: true,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Scope {
    Global,
    Main,
    Func(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Compiler error: {0}")]
    Compiler(String),
    #[error("Line {0}: {1}")]
    User(usize, String),
}