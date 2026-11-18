use super::mir::*;
use std::fmt::Write;

/// Emits clean, unboxed LLVM IR matching the inkwell backend spec
pub struct LlvmIrEmitter;

impl LlvmIrEmitter {
    pub fn emit_module(module: &MirModule) -> String {
        let mut ir = String::new();
        ir.push_str("; ModuleID = 'elaichi_module'\n");
        ir.push_str("source_filename = \"elaichi\"\n");
        ir.push_str("target datalayout = \"e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        ir.push_str("target triple = \"x86_64-pc-windows-msvc\"\n\n");

        // External declarations
        ir.push_str("declare double @sqrt(double)\n");
        ir.push_str("declare double @pow(double, double)\n");
        ir.push_str("declare i64 @time(i64*)\n\n");

        // Function definitions
        for func in &module.functions {
            ir.push_str(&Self::emit_function(func));
            ir.push('\n');
        }

        ir
    }

    pub fn emit_function(func: &MirFunction) -> String {
        let mut ir = String::new();

        // Parameter list
        let params_str = func
            .params
            .iter()
            .map(|(reg, _)| format!("i64 %r{}", reg.0))
            .collect::<Vec<_>>()
            .join(", ");

        let _ = writeln!(ir, "define i64 @{}({}) {{", func.name, params_str);

        for block in &func.blocks {
            let _ = writeln!(ir, "bb{}:", block.id.0);

            for inst in &block.instructions {
                match inst {
                    MirInst::Const { dst, val, .. } => match val {
                        MirConst::Int(n) => {
                            let _ = writeln!(ir, "  %r{} = add i64 0, {}", dst.0, n);
                        }
                        MirConst::Float(f) => {
                            let _ = writeln!(ir, "  %r{} = add i64 0, {}", dst.0, *f as i64);
                        }
                        MirConst::Bool(b) => {
                            let _ = writeln!(ir, "  %r{} = add i64 0, {}", dst.0, if *b { 1 } else { 0 });
                        }
                        MirConst::Str(_) => {
                            let _ = writeln!(ir, "  %r{} = add i64 0, 0", dst.0);
                        }
                    },
                    MirInst::Binary { dst, op, lhs, rhs, .. } => {
                        let op_str = match op {
                            MirBinOp::Add => "add i64",
                            MirBinOp::Sub => "sub i64",
                            MirBinOp::Mul => "mul i64",
                            MirBinOp::Div => "sdiv i64",
                            MirBinOp::Mod => "srem i64",
                            MirBinOp::Eq => "icmp eq i64",
                            MirBinOp::Neq => "icmp ne i64",
                            MirBinOp::Lt => "icmp slt i64",
                            MirBinOp::Lte => "icmp sle i64",
                            MirBinOp::Gt => "icmp sgt i64",
                            MirBinOp::Gte => "icmp sge i64",
                        };

                        if matches!(
                            op,
                            MirBinOp::Eq
                                | MirBinOp::Neq
                                | MirBinOp::Lt
                                | MirBinOp::Lte
                                | MirBinOp::Gt
                                | MirBinOp::Gte
                        ) {
                            // Comparison produces i1, zero-extend to i64
                            let _ = writeln!(
                                ir,
                                "  %cmp_{} = {} %r{}, %r{}",
                                dst.0, op_str, lhs.0, rhs.0
                            );
                            let _ = writeln!(ir, "  %r{} = zext i1 %cmp_{} to i64", dst.0, dst.0);
                        } else {
                            let _ = writeln!(
                                ir,
                                "  %r{} = {} %r{}, %r{}",
                                dst.0, op_str, lhs.0, rhs.0
                            );
                        }
                    }
                    MirInst::Neg { dst, src, .. } => {
                        let _ = writeln!(ir, "  %r{} = sub i64 0, %r{}", dst.0, src.0);
                    }
                    MirInst::Not { dst, src } => {
                        let _ = writeln!(ir, "  %r{} = xor i64 %r{}, 1", dst.0, src.0);
                    }
                    MirInst::Call { dst, callee, args, .. } => {
                        let formatted_args = args
                            .iter()
                            .map(|r| format!("i64 %r{}", r.0))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let _ = writeln!(
                            ir,
                            "  %r{} = call i64 @{}({})",
                            dst.0, callee, formatted_args
                        );
                    }
                    MirInst::AllocStack { dst, bytes, .. } => {
                        let _ = writeln!(ir, "  %r{} = alloca [{} x i8], align 8", dst.0, bytes);
                    }
                    MirInst::Phi { dst, incoming, .. } => {
                        let incoming_str = incoming
                            .iter()
                            .map(|(reg, bb)| format!("[ %r{}, %bb{} ]", reg.0, bb.0))
                            .collect::<Vec<_>>()
                            .join(", ");
                        let _ = writeln!(ir, "  %r{} = phi i64 {}", dst.0, incoming_str);
                    }
                }
            }

            match &block.terminator {
                MirTerminator::Goto(target) => {
                    let _ = writeln!(ir, "  br label %bb{}", target.0);
                }
                MirTerminator::Branch { cond, then_block, else_block } => {
                    let _ = writeln!(ir, "  %bcond_{} = trunc i64 %r{} to i1", cond.0, cond.0);
                    let _ = writeln!(
                        ir,
                        "  br i1 %bcond_{}, label %bb{}, label %bb{}",
                        cond.0, then_block.0, else_block.0
                    );
                }
                MirTerminator::Return(Some(reg)) => {
                    let _ = writeln!(ir, "  ret i64 %r{}", reg.0);
                }
                MirTerminator::Return(None) => {
                    let _ = writeln!(ir, "  ret i64 0");
                }
                MirTerminator::Unreachable => {
                    let _ = writeln!(ir, "  unreachable");
                }
            }
        }

        ir.push_str("}\n");
        ir
    }
}
