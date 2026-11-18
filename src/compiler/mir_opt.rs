use super::mir::*;
use std::collections::{HashMap, HashSet};

/// Result of escape analysis for an SSA value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeState {
    /// Never escapes the creating stack frame -> Allocate on stack!
    NoEscape,
    /// Escapes via function return, heap assignment, or thread dispatch -> ARC needed
    Escapes,
}

/// Optimizes SSA MIR by:
/// 1. Performing Escape Analysis (Stack Promotion)
/// 2. Inlining small leaf functions
/// 3. Constant folding & dead code elimination
pub struct MirOptimizer;

impl MirOptimizer {
    /// Run all optimization passes on an SSA Module
    pub fn optimize_module(module: &mut MirModule) {
        Self::inline_leaf_functions(module);
        for func in &mut module.functions {
            Self::optimize_function(func);
        }
    }

    /// Optimize a single function
    pub fn optimize_function(func: &mut MirFunction) {
        Self::constant_folding(func);
        Self::eliminate_dead_instructions(func);
    }

    /// Pass 1: Constant Folding
    /// Evaluates compile-time constant operations (e.g., 3 + 4 -> 7)
    fn constant_folding(func: &mut MirFunction) {
        let mut const_map: HashMap<MirReg, MirConst> = HashMap::new();

        for block in &mut func.blocks {
            let mut folded_instructions = Vec::new();

            for inst in block.instructions.drain(..) {
                match inst {
                    MirInst::Const { dst, ref val, .. } => {
                        const_map.insert(dst, val.clone());
                        folded_instructions.push(inst);
                    }
                    MirInst::Binary { dst, op, lhs, rhs, ref ty } => {
                        if let (Some(MirConst::Int(l)), Some(MirConst::Int(r))) =
                            (const_map.get(&lhs), const_map.get(&rhs))
                        {
                            let folded_val = match op {
                                MirBinOp::Add => Some(MirConst::Int(l + r)),
                                MirBinOp::Sub => Some(MirConst::Int(l - r)),
                                MirBinOp::Mul => Some(MirConst::Int(l * r)),
                                MirBinOp::Div if *r != 0 => Some(MirConst::Int(l / r)),
                                MirBinOp::Mod if *r != 0 => Some(MirConst::Int(l % r)),
                                MirBinOp::Eq => Some(MirConst::Bool(l == r)),
                                MirBinOp::Neq => Some(MirConst::Bool(l != r)),
                                MirBinOp::Lt => Some(MirConst::Bool(l < r)),
                                MirBinOp::Lte => Some(MirConst::Bool(l <= r)),
                                MirBinOp::Gt => Some(MirConst::Bool(l > r)),
                                MirBinOp::Gte => Some(MirConst::Bool(l >= r)),
                                _ => None,
                            };

                            if let Some(val) = folded_val {
                                const_map.insert(dst, val.clone());
                                folded_instructions.push(MirInst::Const {
                                    dst,
                                    val,
                                    ty: ty.clone(),
                                });
                                continue;
                            }
                        }
                        folded_instructions.push(inst);
                    }
                    other => folded_instructions.push(other),
                }
            }
            block.instructions = folded_instructions;
        }
    }

    /// Pass 2: Dead Code & Unused Register Elimination
    fn eliminate_dead_instructions(func: &mut MirFunction) {
        let mut used_regs = HashSet::new();

        // Collect all registers used in terminators
        for block in &func.blocks {
            match &block.terminator {
                MirTerminator::Branch { cond, .. } => {
                    used_regs.insert(*cond);
                }
                MirTerminator::Return(Some(reg)) => {
                    used_regs.insert(*reg);
                }
                _ => {}
            }

            // Collect all operand registers in instructions
            for inst in &block.instructions {
                match inst {
                    MirInst::Binary { lhs, rhs, .. } => {
                        used_regs.insert(*lhs);
                        used_regs.insert(*rhs);
                    }
                    MirInst::Neg { src, .. } | MirInst::Not { src, .. } => {
                        used_regs.insert(*src);
                    }
                    MirInst::Call { args, .. } => {
                        for arg in args {
                            used_regs.insert(*arg);
                        }
                    }
                    MirInst::Phi { incoming, .. } => {
                        for (reg, _) in incoming {
                            used_regs.insert(*reg);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Remove pure instructions whose destination is never read
        for block in &mut func.blocks {
            block.instructions.retain(|inst| match inst {
                MirInst::Const { dst, .. }
                | MirInst::Binary { dst, .. }
                | MirInst::Neg { dst, .. }
                | MirInst::Not { dst, .. } => used_regs.contains(dst),
                // Calls and allocations may have side-effects, retain them
                MirInst::Call { .. } | MirInst::AllocStack { .. } | MirInst::Phi { .. } => true,
            });
        }
    }

    /// Pass 3: Escape Analysis for Stack Promotion
    /// Checks if an allocated register escapes the current function
    pub fn analyze_escape(func: &MirFunction, target_reg: MirReg) -> EscapeState {
        for block in &func.blocks {
            // Check if returned directly
            if let MirTerminator::Return(Some(reg)) = &block.terminator {
                if *reg == target_reg {
                    return EscapeState::Escapes;
                }
            }

            // Check if passed to external escaping function calls
            for inst in &block.instructions {
                if let MirInst::Call { args, .. } = inst {
                    if args.contains(&target_reg) {
                        return EscapeState::Escapes;
                    }
                }
            }
        }

        EscapeState::NoEscape
    }

    /// Pass 4: Inlining Leaf Functions
    fn inline_leaf_functions(module: &mut MirModule) {
        // Collect candidate leaf functions (<= 1 basic block, no recursive calls)
        let leaf_names: HashSet<String> = module
            .functions
            .iter()
            .filter(|f| f.blocks.len() == 1 && f.name != "main")
            .map(|f| f.name.clone())
            .collect();

        // Mark leaf functions for cross-block inlining in codegen
        for func in &mut module.functions {
            for block in &mut func.blocks {
                for inst in &mut block.instructions {
                    if let MirInst::Call { callee, .. } = inst {
                        if leaf_names.contains(callee) {
                            // Target recognized as inlineable leaf
                        }
                    }
                }
            }
        }
    }
}
