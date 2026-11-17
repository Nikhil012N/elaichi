#[derive(Debug, Clone, PartialEq)]
pub enum ElaichiType {
    I64,
    F64,
    Bool,
    String,
    Dynamic,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TerminalAction {
    Print,
    Println,
    Error,
    Warn,
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    BoolLit(bool),
    Var(String),
    ListLit(Vec<Expr>),
    MapLit(Vec<(String, Expr)>),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        target_ty: ElaichiType,
    },
    AnonymousFn {
        params: Vec<String>,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    LetStatic {
        name: String,
        is_mut: bool,
        ty: ElaichiType,
        init: Expr,
    },
    VarDynamic {
        name: String,
        init: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    IndexAssign {
        target: String,
        index: Expr,
        value: Expr,
    },
    FieldAssign {
        target: String,
        field: String,
        value: Expr,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        var_name: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Return(Option<Expr>),
    TerminalCall {
        action: TerminalAction,
        expr: Option<Expr>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: ElaichiType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Param>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub is_static: bool,
    pub params: Vec<Param>,
    pub return_ty: ElaichiType,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub structs: Vec<StructDef>,
    pub functions: Vec<Function>,
}
