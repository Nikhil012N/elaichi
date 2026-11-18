use crate::ast::ElaichiType;

#[derive(Debug, Clone, PartialEq)]
pub enum StaticValue {
    I64(i64),
    F64(f64),
    Bool(bool),
    StructInstance {
        type_name: String,
        fields: Vec<StaticValue>,
    },
}

impl StaticValue {
    pub fn ty(&self) -> ElaichiType {
        match self {
            StaticValue::I64(_) => ElaichiType::I64,
            StaticValue::F64(_) => ElaichiType::F64,
            StaticValue::Bool(_) => ElaichiType::Bool,
            StaticValue::StructInstance { type_name, .. } => ElaichiType::Custom(type_name.clone()),
        }
    }

    pub fn to_f64(&self) -> Result<f64, String> {
        match self {
            StaticValue::F64(f) => Ok(*f),
            StaticValue::I64(n) => Ok(*n as f64),
            StaticValue::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            _ => Err(format!("Cannot convert static value {:?} to f64", self)),
        }
    }

    pub fn to_i64(&self) -> Result<i64, String> {
        match self {
            StaticValue::I64(n) => Ok(*n),
            StaticValue::F64(f) => Ok(*f as i64),
            StaticValue::Bool(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(format!("Cannot convert static value {:?} to i64", self)),
        }
    }
}

// Low-level unboxed instruction set for the Static Stack
#[derive(Debug, Clone)]
pub enum StaticInst {
    ConstI64(i64),
    ConstF64(f64),
    ConstBool(bool),
    LoadLocal(usize),
    StoreLocal(usize),
    I64Add,
    I64Sub,
    I64Mul,
    I64Div,
    I64Mod,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Eq,
    F64Neq,
    F64Lt,
    F64Le,
    F64Gt,
    F64Ge,
    I64Eq,
    I64Neq,
    I64Lt,
    I64Le,
    I64Gt,
    I64Ge,
    JumpIfFalse(usize),
    Jump(usize),
    Call(String, usize),
    Return,
}

#[derive(Debug, Clone, Default)]
pub struct StaticFunctionCode {
    pub code: Vec<StaticInst>,
    pub local_types: Vec<ElaichiType>,
}
