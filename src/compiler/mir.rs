use super::types::Type;
use std::fmt;

/// Unique identifier for an SSA register / value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirReg(pub usize);

impl fmt::Display for MirReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.0)
    }
}

/// Unique label for a Basic Block in the Control Flow Graph (CFG)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}

/// Constant scalar values for pure unboxed computations
#[derive(Debug, Clone, PartialEq)]
pub enum MirConst {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

/// Binary operations supported in SSA form
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

/// SSA Instructions operating purely on registers and basic blocks
#[derive(Debug, Clone, PartialEq)]
pub enum MirInst {
    /// %dst = const <val> : <type>
    Const {
        dst: MirReg,
        val: MirConst,
        ty: Type,
    },
    /// %dst = binop %lhs, %rhs : <type>
    Binary {
        dst: MirReg,
        op: MirBinOp,
        lhs: MirReg,
        rhs: MirReg,
        ty: Type,
    },
    /// %dst = neg %src
    Neg {
        dst: MirReg,
        src: MirReg,
        ty: Type,
    },
    /// %dst = not %src
    Not {
        dst: MirReg,
        src: MirReg,
    },
    /// %dst = call @func(%arg1, %arg2, ...) : <type>
    Call {
        dst: MirReg,
        callee: String,
        args: Vec<MirReg>,
        ty: Type,
    },
    /// %dst = phi [(%val1, bb1), (%val2, bb2), ...] : <type>
    Phi {
        dst: MirReg,
        incoming: Vec<(MirReg, BlockId)>,
        ty: Type,
    },
    /// %dst = alloc_stack <size_in_bytes>
    AllocStack {
        dst: MirReg,
        bytes: usize,
        ty: Type,
    },
}

/// Basic Block Terminator Instructions (every block MUST end with one)
#[derive(Debug, Clone, PartialEq)]
pub enum MirTerminator {
    /// Unconditional jump to target block
    Goto(BlockId),
    /// Conditional branch based on boolean register
    Branch {
        cond: MirReg,
        then_block: BlockId,
        else_block: BlockId,
    },
    /// Return from current function
    Return(Option<MirReg>),
    /// Unreachable marker for dead paths
    Unreachable,
}

/// Basic Block: linear sequence of non-branching instructions ending with a terminator
#[derive(Debug, Clone)]
pub struct MirBlock {
    pub id: BlockId,
    pub instructions: Vec<MirInst>,
    pub terminator: MirTerminator,
}

/// A fully monomorphized, typed SSA Function
#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<(MirReg, Type)>,
    pub return_ty: Type,
    pub blocks: Vec<MirBlock>,
    pub reg_counter: usize,
}

impl MirFunction {
    pub fn new(name: String, params: Vec<(MirReg, Type)>, return_ty: Type) -> Self {
        Self {
            name,
            params,
            return_ty,
            blocks: Vec::new(),
            reg_counter: 0,
        }
    }

    pub fn fresh_reg(&mut self) -> MirReg {
        let r = MirReg(self.reg_counter);
        self.reg_counter += 1;
        r
    }
}

/// Entire compiled SSA Module ready for LLVM / Cranelift codegen
#[derive(Debug, Clone, Default)]
pub struct MirModule {
    pub functions: Vec<MirFunction>,
}
