use crate::frontend::*;
use super::static_engine::{StaticFunctionCode, StaticInst};
use super::vm::{DynamicBytecodeChunk, DynValue, OpCode};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Bifurcator {
    pub structs: HashMap<String, StructDef>,
    pub static_functions: HashMap<String, (Function, StaticFunctionCode)>,
    pub dynamic_functions: HashMap<String, (Function, DynamicBytecodeChunk)>,
}

impl Bifurcator {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
            static_functions: HashMap::new(),
            dynamic_functions: HashMap::new(),
        }
    }

    pub fn compile_program(&mut self, program: Program) -> Result<(), String> {
        for s in program.structs {
            self.structs.insert(s.name.clone(), s);
        }

        for func in program.functions {
            if func.is_static {
                let code = self.compile_static_func(&func)?;
                self.static_functions.insert(func.name.clone(), (func, code));
            } else {
                let code = self.compile_dynamic_func(&func)?;
                self.dynamic_functions.insert(func.name.clone(), (func, code));
            }
        }
        Ok(())
    }

    fn compile_dynamic_func(&self, func: &Function) -> Result<DynamicBytecodeChunk, String> {
        let mut chunk = DynamicBytecodeChunk::default();
        let mut local_map: HashMap<String, usize> = HashMap::new();

        for param in &func.params {
            let idx = chunk.local_names.len();
            chunk.local_names.push(param.name.clone());
            local_map.insert(param.name.clone(), idx);
        }

        for stmt in &func.body {
            self.compile_dyn_stmt(stmt, &mut chunk, &mut local_map, &mut Vec::new(), &mut Vec::new())?;
        }

        chunk.code.push(OpCode::Push(DynValue::Nil));
        chunk.code.push(OpCode::Return);

        Ok(chunk)
    }

    fn compile_dyn_stmt(
        &self,
        stmt: &Stmt,
        chunk: &mut DynamicBytecodeChunk,
        local_map: &mut HashMap<String, usize>,
        break_jumps: &mut Vec<usize>,
        continue_jumps: &mut Vec<usize>,
    ) -> Result<(), String> {
        match stmt {
            Stmt::LetStatic { name, init, .. } | Stmt::VarDynamic { name, init } => {
                self.compile_dyn_expr(init, chunk, local_map)?;
                let idx = chunk.local_names.len();
                chunk.local_names.push(name.clone());
                local_map.insert(name.clone(), idx);
                chunk.code.push(OpCode::StoreLocal(idx));
            }
            Stmt::Assign { name, expr } => {
                self.compile_dyn_expr(expr, chunk, local_map)?;
                if let Some(&idx) = local_map.get(name) {
                    chunk.code.push(OpCode::StoreLocal(idx));
                } else {
                    chunk.code.push(OpCode::StoreGlobal(name.clone()));
                }
            }
            Stmt::IndexAssign { target, index, value } => {
                if let Some(&idx) = local_map.get(target) {
                    chunk.code.push(OpCode::LoadLocal(idx));
                } else {
                    chunk.code.push(OpCode::LoadGlobal(target.clone()));
                }
                self.compile_dyn_expr(index, chunk, local_map)?;
                self.compile_dyn_expr(value, chunk, local_map)?;
                chunk.code.push(OpCode::IndexSet);
            }
            Stmt::FieldAssign { target, field, value } => {
                if let Some(&idx) = local_map.get(target) {
                    chunk.code.push(OpCode::LoadLocal(idx));
                } else {
                    chunk.code.push(OpCode::LoadGlobal(target.clone()));
                }
                self.compile_dyn_expr(value, chunk, local_map)?;
                chunk.code.push(OpCode::FieldSet(field.clone()));
                if let Some(&idx) = local_map.get(target) {
                    chunk.code.push(OpCode::StoreLocal(idx));
                } else {
                    chunk.code.push(OpCode::StoreGlobal(target.clone()));
                }
            }
            Stmt::TerminalCall { action, expr } => {
                if let Some(e) = expr {
                    self.compile_dyn_expr(e, chunk, local_map)?;
                } else {
                    chunk.code.push(OpCode::Push(DynValue::Nil));
                }

                match action {
                    TerminalAction::Print => chunk.code.push(OpCode::TerminalPrint),
                    TerminalAction::Println => chunk.code.push(OpCode::TerminalPrintln),
                    TerminalAction::Error => chunk.code.push(OpCode::TerminalError),
                    TerminalAction::Warn => chunk.code.push(OpCode::TerminalWarn),
                    TerminalAction::Clear => chunk.code.push(OpCode::TerminalClear),
                }
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_dyn_expr(expr, chunk, local_map)?;
                } else {
                    chunk.code.push(OpCode::Push(DynValue::Nil));
                }
                chunk.code.push(OpCode::Return);
            }
            Stmt::Break => {
                let jump_idx = chunk.code.len();
                chunk.code.push(OpCode::Jump(0));
                break_jumps.push(jump_idx);
            }
            Stmt::Continue => {
                let jump_idx = chunk.code.len();
                chunk.code.push(OpCode::Jump(0));
                continue_jumps.push(jump_idx);
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.compile_dyn_expr(cond, chunk, local_map)?;
                let jump_false_idx = chunk.code.len();
                chunk.code.push(OpCode::JumpIfFalse(0));

                for s in then_branch {
                    self.compile_dyn_stmt(s, chunk, local_map, break_jumps, continue_jumps)?;
                }

                if let Some(eb) = else_branch {
                    let jump_end_idx = chunk.code.len();
                    chunk.code.push(OpCode::Jump(0));

                    let else_start = chunk.code.len();
                    chunk.code[jump_false_idx] = OpCode::JumpIfFalse(else_start);

                    for s in eb {
                        self.compile_dyn_stmt(s, chunk, local_map, break_jumps, continue_jumps)?;
                    }

                    let end_pos = chunk.code.len();
                    chunk.code[jump_end_idx] = OpCode::Jump(end_pos);
                } else {
                    let end_pos = chunk.code.len();
                    chunk.code[jump_false_idx] = OpCode::JumpIfFalse(end_pos);
                }
            }
            Stmt::While { cond, body } => {
                let loop_start = chunk.code.len();
                self.compile_dyn_expr(cond, chunk, local_map)?;
                let exit_jump_idx = chunk.code.len();
                chunk.code.push(OpCode::JumpIfFalse(0));

                let mut loop_breaks = Vec::new();
                let mut loop_continues = Vec::new();

                for s in body {
                    self.compile_dyn_stmt(s, chunk, local_map, &mut loop_breaks, &mut loop_continues)?;
                }

                chunk.code.push(OpCode::Jump(loop_start));
                let exit_pos = chunk.code.len();
                chunk.code[exit_jump_idx] = OpCode::JumpIfFalse(exit_pos);

                for b in loop_breaks {
                    chunk.code[b] = OpCode::Jump(exit_pos);
                }
                for c in loop_continues {
                    chunk.code[c] = OpCode::Jump(loop_start);
                }
            }
            Stmt::For { var_name, start, end, body } => {
                self.compile_dyn_expr(start, chunk, local_map)?;
                let var_idx = chunk.local_names.len();
                chunk.local_names.push(var_name.clone());
                local_map.insert(var_name.clone(), var_idx);
                chunk.code.push(OpCode::StoreLocal(var_idx));

                let loop_start = chunk.code.len();
                chunk.code.push(OpCode::LoadLocal(var_idx));
                self.compile_dyn_expr(end, chunk, local_map)?;
                chunk.code.push(OpCode::Less);
                let exit_jump_idx = chunk.code.len();
                chunk.code.push(OpCode::JumpIfFalse(0));

                let mut loop_breaks = Vec::new();
                let mut loop_continues = Vec::new();

                for s in body {
                    self.compile_dyn_stmt(s, chunk, local_map, &mut loop_breaks, &mut loop_continues)?;
                }

                let continue_target = chunk.code.len();
                chunk.code.push(OpCode::LoadLocal(var_idx));
                chunk.code.push(OpCode::Push(DynValue::Int(1)));
                chunk.code.push(OpCode::Add);
                chunk.code.push(OpCode::StoreLocal(var_idx));

                chunk.code.push(OpCode::Jump(loop_start));
                let exit_pos = chunk.code.len();
                chunk.code[exit_jump_idx] = OpCode::JumpIfFalse(exit_pos);

                for b in loop_breaks {
                    chunk.code[b] = OpCode::Jump(exit_pos);
                }
                for c in loop_continues {
                    chunk.code[c] = OpCode::Jump(continue_target);
                }
            }
            Stmt::Expr(expr) => {
                self.compile_dyn_expr(expr, chunk, local_map)?;
            }
        }
        Ok(())
    }

    fn compile_dyn_expr(
        &self,
        expr: &Expr,
        chunk: &mut DynamicBytecodeChunk,
        local_map: &HashMap<String, usize>,
    ) -> Result<(), String> {
        match expr {
            Expr::IntLit(n) => chunk.code.push(OpCode::Push(DynValue::Int(*n))),
            Expr::FloatLit(f) => chunk.code.push(OpCode::Push(DynValue::Float(*f))),
            Expr::StringLit(s) => chunk.code.push(OpCode::Push(DynValue::Str(s.clone()))),
            Expr::BoolLit(b) => chunk.code.push(OpCode::Push(DynValue::Bool(*b))),
            Expr::ListLit(items) => {
                for item in items {
                    self.compile_dyn_expr(item, chunk, local_map)?;
                }
                chunk.code.push(OpCode::BuildList(items.len()));
            }
            Expr::MapLit(pairs) => {
                for (k, v) in pairs {
                    chunk.code.push(OpCode::Push(DynValue::Str(k.clone())));
                    self.compile_dyn_expr(v, chunk, local_map)?;
                }
                chunk.code.push(OpCode::BuildMap(pairs.len()));
            }
            Expr::Cast { expr, target_ty } => {
                self.compile_dyn_expr(expr, chunk, local_map)?;
                chunk.code.push(OpCode::Cast(target_ty.clone()));
            }
            Expr::AnonymousFn { .. } => {
                return Err("Anonymous functions are experimental in dynamic bytecode".to_string());
            }
            Expr::Index { target, index } => {
                self.compile_dyn_expr(target, chunk, local_map)?;
                self.compile_dyn_expr(index, chunk, local_map)?;
                chunk.code.push(OpCode::IndexGet);
            }
            Expr::FieldAccess { target, field } => {
                self.compile_dyn_expr(target, chunk, local_map)?;
                chunk.code.push(OpCode::FieldGet(field.clone()));
            }
            Expr::Unary { op, expr } => {
                self.compile_dyn_expr(expr, chunk, local_map)?;
                match op {
                    UnaryOp::Not => chunk.code.push(OpCode::Not),
                    UnaryOp::Neg => chunk.code.push(OpCode::Neg),
                }
            }
            Expr::Var(name) => {
                if let Some(&idx) = local_map.get(name) {
                    chunk.code.push(OpCode::LoadLocal(idx));
                } else {
                    chunk.code.push(OpCode::LoadGlobal(name.clone()));
                }
            }
            Expr::Binary { op, left, right } => {
                self.compile_dyn_expr(left, chunk, local_map)?;
                self.compile_dyn_expr(right, chunk, local_map)?;
                match op {
                    BinaryOp::Add => chunk.code.push(OpCode::Add),
                    BinaryOp::Sub => chunk.code.push(OpCode::Sub),
                    BinaryOp::Mul => chunk.code.push(OpCode::Mul),
                    BinaryOp::Div => chunk.code.push(OpCode::Div),
                    BinaryOp::Mod => chunk.code.push(OpCode::Mod),
                    BinaryOp::Equal => chunk.code.push(OpCode::Equal),
                    BinaryOp::NotEqual => chunk.code.push(OpCode::NotEqual),
                    BinaryOp::Less => chunk.code.push(OpCode::Less),
                    BinaryOp::LessEqual => chunk.code.push(OpCode::LessEqual),
                    BinaryOp::Greater => chunk.code.push(OpCode::Greater),
                    BinaryOp::GreaterEqual => chunk.code.push(OpCode::GreaterEqual),
                    BinaryOp::And => chunk.code.push(OpCode::And),
                    BinaryOp::Or => chunk.code.push(OpCode::Or),
                }
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_dyn_expr(arg, chunk, local_map)?;
                }
                chunk.code.push(OpCode::Call(callee.clone(), args.len()));
            }
        }
        Ok(())
    }

    fn compile_static_func(&self, func: &Function) -> Result<StaticFunctionCode, String> {
        let mut code = StaticFunctionCode::default();
        let mut local_map: HashMap<String, usize> = HashMap::new();

        for param in &func.params {
            let idx = code.local_types.len();
            code.local_types.push(param.ty.clone());
            local_map.insert(param.name.clone(), idx);
        }

        for stmt in &func.body {
            self.compile_static_stmt(stmt, &mut code, &mut local_map)?;
        }

        code.code.push(StaticInst::ConstI64(0));
        code.code.push(StaticInst::Return);

        Ok(code)
    }

    fn compile_static_stmt(
        &self,
        stmt: &Stmt,
        code: &mut StaticFunctionCode,
        local_map: &mut HashMap<String, usize>,
    ) -> Result<(), String> {
        match stmt {
            Stmt::LetStatic { name, ty, init, .. } => {
                self.compile_static_expr(init, code, local_map)?;
                let idx = code.local_types.len();
                code.local_types.push(ty.clone());
                local_map.insert(name.clone(), idx);
                code.code.push(StaticInst::StoreLocal(idx));
            }
            Stmt::Assign { name, expr } => {
                self.compile_static_expr(expr, code, local_map)?;
                if let Some(&idx) = local_map.get(name) {
                    code.code.push(StaticInst::StoreLocal(idx));
                } else {
                    return Err(format!("Unknown static variable '{}'", name));
                }
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_static_expr(expr, code, local_map)?;
                } else {
                    code.code.push(StaticInst::ConstI64(0));
                }
                code.code.push(StaticInst::Return);
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.compile_static_expr(cond, code, local_map)?;
                let jump_false_idx = code.code.len();
                code.code.push(StaticInst::JumpIfFalse(0));

                for s in then_branch {
                    self.compile_static_stmt(s, code, local_map)?;
                }

                if let Some(eb) = else_branch {
                    let jump_end_idx = code.code.len();
                    code.code.push(StaticInst::Jump(0));

                    let else_start = code.code.len();
                    code.code[jump_false_idx] = StaticInst::JumpIfFalse(else_start);

                    for s in eb {
                        self.compile_static_stmt(s, code, local_map)?;
                    }

                    let end_pos = code.code.len();
                    code.code[jump_end_idx] = StaticInst::Jump(end_pos);
                } else {
                    let end_pos = code.code.len();
                    code.code[jump_false_idx] = StaticInst::JumpIfFalse(end_pos);
                }
            }
            Stmt::While { cond, body } => {
                let loop_start = code.code.len();
                self.compile_static_expr(cond, code, local_map)?;
                let exit_jump_idx = code.code.len();
                code.code.push(StaticInst::JumpIfFalse(0));

                for s in body {
                    self.compile_static_stmt(s, code, local_map)?;
                }

                code.code.push(StaticInst::Jump(loop_start));
                let exit_pos = code.code.len();
                code.code[exit_jump_idx] = StaticInst::JumpIfFalse(exit_pos);
            }
            Stmt::For { var_name, start, end, body } => {
                self.compile_static_expr(start, code, local_map)?;
                let var_idx = code.local_types.len();
                code.local_types.push(ElaichiType::I64);
                local_map.insert(var_name.clone(), var_idx);
                code.code.push(StaticInst::StoreLocal(var_idx));

                let loop_start = code.code.len();
                code.code.push(StaticInst::LoadLocal(var_idx));
                self.compile_static_expr(end, code, local_map)?;
                code.code.push(StaticInst::I64Lt);
                let exit_jump_idx = code.code.len();
                code.code.push(StaticInst::JumpIfFalse(0));

                for s in body {
                    self.compile_static_stmt(s, code, local_map)?;
                }

                code.code.push(StaticInst::LoadLocal(var_idx));
                code.code.push(StaticInst::ConstI64(1));
                code.code.push(StaticInst::I64Add);
                code.code.push(StaticInst::StoreLocal(var_idx));

                code.code.push(StaticInst::Jump(loop_start));
                let exit_pos = code.code.len();
                code.code[exit_jump_idx] = StaticInst::JumpIfFalse(exit_pos);
            }
            Stmt::Expr(expr) => {
                self.compile_static_expr(expr, code, local_map)?;
            }
            _ => {
                return Err("Terminal statements cannot be directly embedded in pure static routines".to_string());
            }
        }
        Ok(())
    }

    fn compile_static_expr(
        &self,
        expr: &Expr,
        code: &mut StaticFunctionCode,
        local_map: &HashMap<String, usize>,
    ) -> Result<(), String> {
        match expr {
            Expr::IntLit(n) => code.code.push(StaticInst::ConstI64(*n)),
            Expr::FloatLit(f) => code.code.push(StaticInst::ConstF64(*f)),
            Expr::BoolLit(b) => code.code.push(StaticInst::ConstBool(*b)),
            Expr::Var(name) => {
                if let Some(&idx) = local_map.get(name) {
                    code.code.push(StaticInst::LoadLocal(idx));
                } else {
                    return Err(format!("Unknown static variable '{}'", name));
                }
            }
            Expr::Binary { op, left, right } => {
                self.compile_static_expr(left, code, local_map)?;
                self.compile_static_expr(right, code, local_map)?;
                let is_float = self.expr_is_float(left, local_map) || self.expr_is_float(right, local_map);
                if is_float {
                    match op {
                        BinaryOp::Add => code.code.push(StaticInst::F64Add),
                        BinaryOp::Sub => code.code.push(StaticInst::F64Sub),
                        BinaryOp::Mul => code.code.push(StaticInst::F64Mul),
                        BinaryOp::Div => code.code.push(StaticInst::F64Div),
                        BinaryOp::Equal => code.code.push(StaticInst::F64Eq),
                        BinaryOp::NotEqual => code.code.push(StaticInst::F64Neq),
                        BinaryOp::Less => code.code.push(StaticInst::F64Lt),
                        BinaryOp::LessEqual => code.code.push(StaticInst::F64Le),
                        BinaryOp::Greater => code.code.push(StaticInst::F64Gt),
                        BinaryOp::GreaterEqual => code.code.push(StaticInst::F64Ge),
                        _ => return Err(format!("Unsupported float op in static expr: {:?}", op)),
                    }
                } else {
                    match op {
                        BinaryOp::Add => code.code.push(StaticInst::I64Add),
                        BinaryOp::Sub => code.code.push(StaticInst::I64Sub),
                        BinaryOp::Mul => code.code.push(StaticInst::I64Mul),
                        BinaryOp::Div => code.code.push(StaticInst::I64Div),
                        BinaryOp::Mod => code.code.push(StaticInst::I64Mod),
                        BinaryOp::Equal => code.code.push(StaticInst::I64Eq),
                        BinaryOp::NotEqual => code.code.push(StaticInst::I64Neq),
                        BinaryOp::Less => code.code.push(StaticInst::I64Lt),
                        BinaryOp::LessEqual => code.code.push(StaticInst::I64Le),
                        BinaryOp::Greater => code.code.push(StaticInst::I64Gt),
                        BinaryOp::GreaterEqual => code.code.push(StaticInst::I64Ge),
                        _ => return Err(format!("Unsupported binary op in static expr: {:?}", op)),
                    }
                }
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_static_expr(arg, code, local_map)?;
                }
                code.code.push(StaticInst::Call(callee.clone(), args.len()));
            }
            _ => return Err(format!("Invalid static expression: {:?}", expr)),
        }
        Ok(())
    }

    fn expr_is_float(&self, expr: &Expr, local_map: &HashMap<String, usize>) -> bool {
        match expr {
            Expr::FloatLit(_) => true,
            Expr::Var(name) => {
                if let Some(&_idx) = local_map.get(name) {
                    false
                } else {
                    false
                }
            }
            Expr::Binary { left, right, .. } => {
                self.expr_is_float(left, local_map) || self.expr_is_float(right, local_map)
            }
            _ => false,
        }
    }
}
