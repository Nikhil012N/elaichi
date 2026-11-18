use super::mir::*;
use super::types::{Type, TypeEnv};
use crate::frontend::ast::{BinaryOp, Expr, Function, Stmt, UnaryOp};
use std::collections::HashMap;

/// Lowers typed AST into SSA Basic Blocks with register assignments
pub struct MirBuilder {
    pub current_func: MirFunction,
    current_block: BlockId,
    block_counter: usize,
    var_map: HashMap<String, MirReg>,
}

impl MirBuilder {
    pub fn new(name: String, params: Vec<(String, Type)>, return_ty: Type) -> Self {
        let mut func = MirFunction::new(name, Vec::new(), return_ty);
        let mut var_map = HashMap::new();

        for (p_name, p_ty) in params {
            let reg = func.fresh_reg();
            func.params.push((reg, p_ty));
            var_map.insert(p_name, reg);
        }

        let entry_block = BlockId(0);
        func.blocks.push(MirBlock {
            id: entry_block,
            instructions: Vec::new(),
            terminator: MirTerminator::Unreachable,
        });

        Self {
            current_func: func,
            current_block: entry_block,
            block_counter: 1,
            var_map,
        }
    }

    pub fn create_block(&mut self) -> BlockId {
        let id = BlockId(self.block_counter);
        self.block_counter += 1;
        self.current_func.blocks.push(MirBlock {
            id,
            instructions: Vec::new(),
            terminator: MirTerminator::Unreachable,
        });
        id
    }

    pub fn set_current_block(&mut self, block: BlockId) {
        self.current_block = block;
    }

    pub fn emit_inst(&mut self, inst: MirInst) {
        if let Some(block) = self.current_func.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.instructions.push(inst);
        }
    }

    pub fn set_terminator(&mut self, term: MirTerminator) {
        if let Some(block) = self.current_func.blocks.iter_mut().find(|b| b.id == self.current_block) {
            block.terminator = term;
        }
    }

    /// Lower an expression into SSA register evaluation
    pub fn lower_expr(&mut self, expr: &Expr, env: &TypeEnv) -> Result<(MirReg, Type), String> {
        match expr {
            Expr::IntLit(val) => {
                let dst = self.current_func.fresh_reg();
                self.emit_inst(MirInst::Const {
                    dst,
                    val: MirConst::Int(*val),
                    ty: Type::Int,
                });
                Ok((dst, Type::Int))
            }
            Expr::FloatLit(val) => {
                let dst = self.current_func.fresh_reg();
                self.emit_inst(MirInst::Const {
                    dst,
                    val: MirConst::Float(*val),
                    ty: Type::Float,
                });
                Ok((dst, Type::Float))
            }
            Expr::BoolLit(val) => {
                let dst = self.current_func.fresh_reg();
                self.emit_inst(MirInst::Const {
                    dst,
                    val: MirConst::Bool(*val),
                    ty: Type::Bool,
                });
                Ok((dst, Type::Bool))
            }
            Expr::StringLit(val) => {
                let dst = self.current_func.fresh_reg();
                self.emit_inst(MirInst::Const {
                    dst,
                    val: MirConst::Str(val.clone()),
                    ty: Type::Str,
                });
                Ok((dst, Type::Str))
            }
            Expr::Var(name) => match self.var_map.get(name) {
                Some(reg) => Ok((*reg, Type::Int)), // Resolved statically
                None => Err(format!("SSA lower error: undefined variable '{}'", name)),
            },
            Expr::Unary { op, expr } => {
                let (src, ty) = self.lower_expr(expr, env)?;
                let dst = self.current_func.fresh_reg();
                match op {
                    UnaryOp::Neg => {
                        self.emit_inst(MirInst::Neg { dst, src, ty: ty.clone() });
                        Ok((dst, ty))
                    }
                    UnaryOp::Not => {
                        self.emit_inst(MirInst::Not { dst, src });
                        Ok((dst, Type::Bool))
                    }
                }
            }
            Expr::Binary { op, left, right } => {
                let (lhs, lty) = self.lower_expr(left, env)?;
                let (rhs, _) = self.lower_expr(right, env)?;
                let dst = self.current_func.fresh_reg();

                let (mir_op, res_ty) = match op {
                    BinaryOp::Add => (MirBinOp::Add, lty),
                    BinaryOp::Sub => (MirBinOp::Sub, lty),
                    BinaryOp::Mul => (MirBinOp::Mul, lty),
                    BinaryOp::Div => (MirBinOp::Div, lty),
                    BinaryOp::Mod => (MirBinOp::Mod, lty),
                    BinaryOp::Equal => (MirBinOp::Eq, Type::Bool),
                    BinaryOp::NotEqual => (MirBinOp::Neq, Type::Bool),
                    BinaryOp::Less => (MirBinOp::Lt, Type::Bool),
                    BinaryOp::LessEqual => (MirBinOp::Lte, Type::Bool),
                    BinaryOp::Greater => (MirBinOp::Gt, Type::Bool),
                    BinaryOp::GreaterEqual => (MirBinOp::Gte, Type::Bool),
                    BinaryOp::And => (MirBinOp::Mul, Type::Bool),
                    BinaryOp::Or => (MirBinOp::Add, Type::Bool),
                };

                self.emit_inst(MirInst::Binary {
                    dst,
                    op: mir_op,
                    lhs,
                    rhs,
                    ty: res_ty.clone(),
                });
                Ok((dst, res_ty))
            }
            Expr::Call { callee, args } => {
                let mut reg_args = Vec::new();
                for arg in args {
                    let (reg, _) = self.lower_expr(arg, env)?;
                    reg_args.push(reg);
                }
                let dst = self.current_func.fresh_reg();
                self.emit_inst(MirInst::Call {
                    dst,
                    callee: callee.clone(),
                    args: reg_args,
                    ty: Type::Int,
                });
                Ok((dst, Type::Int))
            }
            _ => {
                let dst = self.current_func.fresh_reg();
                Ok((dst, Type::Void))
            }
        }
    }

    /// Lower function statement list into CFG blocks
    pub fn lower_stmts(&mut self, stmts: &[Stmt], env: &TypeEnv) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::LetStatic { name, init, .. } | Stmt::VarDynamic { name, init } => {
                    let (reg, _) = self.lower_expr(init, env)?;
                    self.var_map.insert(name.clone(), reg);
                }
                Stmt::Assign { name, expr } => {
                    let (reg, _) = self.lower_expr(expr, env)?;
                    self.var_map.insert(name.clone(), reg);
                }
                Stmt::Return(expr_opt) => {
                    if let Some(expr) = expr_opt {
                        let (reg, _) = self.lower_expr(expr, env)?;
                        self.set_terminator(MirTerminator::Return(Some(reg)));
                    } else {
                        self.set_terminator(MirTerminator::Return(None));
                    }
                }
                Stmt::If { cond, then_branch, else_branch } => {
                    let (cond_reg, _) = self.lower_expr(cond, env)?;
                    let then_bb = self.create_block();
                    let else_bb = self.create_block();
                    let merge_bb = self.create_block();

                    let else_target = if else_branch.is_some() { else_bb } else { merge_bb };
                    self.set_terminator(MirTerminator::Branch {
                        cond: cond_reg,
                        then_block: then_bb,
                        else_block: else_target,
                    });

                    // Build then branch
                    self.set_current_block(then_bb);
                    self.lower_stmts(then_branch, env)?;
                    self.set_terminator(MirTerminator::Goto(merge_bb));

                    // Build else branch if present
                    if let Some(else_stmts) = else_branch {
                        self.set_current_block(else_bb);
                        self.lower_stmts(else_stmts, env)?;
                        self.set_terminator(MirTerminator::Goto(merge_bb));
                    }

                    self.set_current_block(merge_bb);
                }
                Stmt::While { cond, body } => {
                    let header_bb = self.create_block();
                    let body_bb = self.create_block();
                    let exit_bb = self.create_block();

                    self.set_terminator(MirTerminator::Goto(header_bb));

                    // Header condition evaluation
                    self.set_current_block(header_bb);
                    let (cond_reg, _) = self.lower_expr(cond, env)?;
                    self.set_terminator(MirTerminator::Branch {
                        cond: cond_reg,
                        then_block: body_bb,
                        else_block: exit_bb,
                    });

                    // Loop body
                    self.set_current_block(body_bb);
                    self.lower_stmts(body, env)?;
                    self.set_terminator(MirTerminator::Goto(header_bb));

                    self.set_current_block(exit_bb);
                }
                Stmt::Expr(expr) => {
                    self.lower_expr(expr, env)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> MirFunction {
        // Ensure every block has a terminator
        for block in &mut self.current_func.blocks {
            if block.terminator == MirTerminator::Unreachable && block.instructions.is_empty() {
                block.terminator = MirTerminator::Return(None);
            }
        }
        self.current_func
    }
}

/// Monomorphization & MIR compilation pass
pub fn compile_function_to_mir(func: &Function, env: &TypeEnv) -> Result<MirFunction, String> {
    let params: Vec<(String, Type)> = func
        .params
        .iter()
        .map(|p| (p.name.clone(), Type::Int))
        .collect();

    let mut builder = MirBuilder::new(func.name.clone(), params, Type::Int);
    builder.lower_stmts(&func.body, env)?;
    Ok(builder.finish())
}
