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
