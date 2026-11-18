use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DynValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<DynValue>),
    Map(HashMap<String, DynValue>),
}

impl DynValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            DynValue::Nil => false,
            DynValue::Bool(b) => *b,
            DynValue::Int(n) => *n != 0,
            DynValue::Float(f) => *f != 0.0,
            DynValue::Str(s) => !s.is_empty(),
            DynValue::List(l) => !l.is_empty(),
            DynValue::Map(m) => !m.is_empty(),
        }
    }

    pub fn to_i64(&self) -> Result<i64, String> {
        match self {
            DynValue::Int(n) => Ok(*n),
            DynValue::Float(f) => Ok(*f as i64),
            DynValue::Bool(b) => Ok(if *b { 1 } else { 0 }),
            _ => Err(format!("Cannot convert dynamic value {:?} to i64", self)),
        }
    }

    pub fn to_f64(&self) -> Result<f64, String> {
        match self {
            DynValue::Float(f) => Ok(*f),
            DynValue::Int(n) => Ok(*n as f64),
            _ => Err(format!("Cannot convert dynamic value {:?} to f64", self)),
        }
    }
}

impl std::fmt::Display for DynValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynValue::Nil => write!(f, "nil"),
            DynValue::Bool(b) => write!(f, "{}", b),
            DynValue::Int(n) => write!(f, "{}", n),
            DynValue::Float(fl) => write!(f, "{}", fl),
            DynValue::Str(s) => write!(f, "{}", s),
            DynValue::List(l) => {
                write!(f, "[")?;
                for (i, item) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            DynValue::Map(m) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in m {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

// OpCodes for the Dynamic VM Stack
#[derive(Debug, Clone)]
pub enum OpCode {
    Push(DynValue),
    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),
    BuildList(usize),
    BuildMap(usize),
    IndexGet,
    IndexSet,
    FieldGet(String),
    FieldSet(String),
    Cast(crate::frontend::ElaichiType),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Neg,
    TerminalPrint,   // print without newline
    TerminalPrintln, // print with newline
    TerminalError,   // print error prefix
    TerminalWarn,    // print warning prefix
    TerminalClear,   // clear terminal
    JumpIfFalse(usize),
    Jump(usize),
    Call(String, usize),
    Return,
}

#[derive(Debug, Clone, Default)]
pub struct DynamicBytecodeChunk {
    pub code: Vec<OpCode>,
    pub local_names: Vec<String>,
}
