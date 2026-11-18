use crate::compiler::{Bifurcator, Bridge, DynValue, OpCode, StaticInst, StaticValue};
use std::collections::HashMap;

pub struct Runtime {
    pub bifurcator: Bifurcator,
    pub dyn_globals: HashMap<String, DynValue>,
}

impl Runtime {
    pub fn new(bifurcator: Bifurcator) -> Self {
        Self {
            bifurcator,
            dyn_globals: HashMap::new(),
        }
    }

    pub fn execute(&mut self, entry_function: &str) -> Result<DynValue, String> {
        if self.bifurcator.dynamic_functions.contains_key(entry_function) {
            self.call_dynamic(entry_function, vec![])
        } else if self.bifurcator.static_functions.contains_key(entry_function) {
            let res = self.call_static(entry_function, vec![])?;
            Ok(Bridge::static_to_dynamic(&res))
        } else {
            Err(format!("Entry function '{}' not found", entry_function))
        }
    }

    pub fn call_dynamic(&mut self, name: &str, args: Vec<DynValue>) -> Result<DynValue, String> {
        let (_func_def, chunk) = match self.bifurcator.dynamic_functions.get(name) {
            Some(pair) => (pair.0.clone(), pair.1.clone()),
            None => {
                if let Some(res) = self.call_stdlib(name, &args)? {
                    return Ok(res);
                }

                if self.bifurcator.static_functions.contains_key(name) {
                    let static_def = self.bifurcator.static_functions[name].0.clone();
                    let mut static_args = Vec::new();
                    for (i, arg) in args.iter().enumerate() {
                        let param_ty = &static_def.params[i].ty;
                        let unboxed = Bridge::dynamic_to_static(arg, param_ty)?;
                        static_args.push(unboxed);
                    }
                    let static_res = self.call_static(name, static_args)?;
                    return Ok(Bridge::static_to_dynamic(&static_res));
                }
                return Err(format!("Undefined function '{}'", name));
            }
        };

        let mut locals = vec![DynValue::Nil; chunk.local_names.len()];
        for (i, arg) in args.into_iter().enumerate() {
            if i < locals.len() {
                locals[i] = arg;
            }
        }

        let mut eval_stack: Vec<DynValue> = Vec::new();
        let mut ip = 0;

        while ip < chunk.code.len() {
            match &chunk.code[ip] {
                OpCode::Push(v) => {
                    eval_stack.push(v.clone());
                    ip += 1;
                }
                OpCode::LoadLocal(idx) => {
                    eval_stack.push(locals[*idx].clone());
                    ip += 1;
                }
                OpCode::StoreLocal(idx) => {
                    let v = eval_stack.pop().unwrap_or(DynValue::Nil);
                    locals[*idx] = v;
                    ip += 1;
                }
                OpCode::LoadGlobal(name) => {
                    let v = self.dyn_globals.get(name).cloned().unwrap_or(DynValue::Nil);
                    eval_stack.push(v);
                    ip += 1;
                }
                OpCode::StoreGlobal(name) => {
                    let v = eval_stack.pop().unwrap_or(DynValue::Nil);
                    self.dyn_globals.insert(name.clone(), v);
                    ip += 1;
                }
                OpCode::BuildList(count) => {
                    let mut items = Vec::new();
                    for _ in 0..*count {
                        items.push(eval_stack.pop().unwrap());
                    }
                    items.reverse();
                    eval_stack.push(DynValue::List(items));
                    ip += 1;
                }
                OpCode::BuildMap(count) => {
                    let mut pairs = Vec::with_capacity(*count);
                    for _ in 0..*count {
                        let val = eval_stack.pop().unwrap();
                        let key = eval_stack.pop().unwrap();
                        let key_str = match key {
                            DynValue::Str(s) => s,
                            other => format!("{}", other),
                        };
                        pairs.push((key_str, val));
                    }
                    pairs.reverse();
                    let mut map = HashMap::new();
                    for (k, v) in pairs {
                        map.insert(k, v);
                    }
                    eval_stack.push(DynValue::Map(map));
                    ip += 1;
                }
                OpCode::IndexGet => {
                    let idx_val = eval_stack.pop().unwrap();
                    let target = eval_stack.pop().unwrap();
                    match (target, idx_val) {
                        (DynValue::List(l), DynValue::Int(idx)) => {
                            let i = idx as usize;
                            if i < l.len() {
                                eval_stack.push(l[i].clone());
                            } else {
                                return Err(format!("Index out of bounds: {} (len {})", i, l.len()));
                            }
                        }
                        (DynValue::Map(m), DynValue::Str(k)) => {
                            let v = m.get(&k).cloned().unwrap_or(DynValue::Nil);
                            eval_stack.push(v);
                        }
                        (DynValue::Str(s), DynValue::Int(idx)) => {
                            let i = idx as usize;
                            if let Some(c) = s.chars().nth(i) {
                                eval_stack.push(DynValue::Str(c.to_string()));
                            } else {
                                return Err(format!("String index out of bounds: {}", i));
                            }
                        }
                        (t, i) => return Err(format!("Cannot index {:?} with {:?}", t, i)),
                    }
                    ip += 1;
                }
                OpCode::IndexSet => {
                    let val = eval_stack.pop().unwrap();
                    let idx_val = eval_stack.pop().unwrap();
                    let mut target = eval_stack.pop().unwrap();
                    match (&mut target, idx_val) {
                        (DynValue::List(ref mut l), DynValue::Int(idx)) => {
                            let i = idx as usize;
                            if i < l.len() {
                                l[i] = val;
                            } else {
                                return Err(format!("Index out of bounds: {}", i));
                            }
                        }
                        (DynValue::Map(ref mut m), DynValue::Str(k)) => {
                            m.insert(k, val);
                        }
                        (t, i) => return Err(format!("Cannot index set {:?} with {:?}", t, i)),
                    }
                    ip += 1;
                }
                OpCode::FieldGet(field) => {
                    let target = eval_stack.pop().unwrap();
                    match target {
                        DynValue::Map(m) => {
                            let v = m.get(field).cloned().unwrap_or(DynValue::Nil);
                            eval_stack.push(v);
                        }
                        DynValue::List(l) if field == "length" || field == "len" => {
                            eval_stack.push(DynValue::Int(l.len() as i64));
                        }
                        DynValue::Str(s) if field == "length" || field == "len" => {
                            eval_stack.push(DynValue::Int(s.len() as i64));
                        }
                        other => return Err(format!("Cannot access field '{}' on {:?}", field, other)),
                    }
                    ip += 1;
                }
                OpCode::FieldSet(field) => {
                    let val = eval_stack.pop().unwrap();
                    let mut target = eval_stack.pop().unwrap();
                    match &mut target {
                        DynValue::Map(ref mut m) => {
                            m.insert(field.clone(), val);
                        }
                        other => return Err(format!("Cannot set field '{}' on {:?}", field, other)),
                    }
                    eval_stack.push(target);
                    ip += 1;
                }
                OpCode::Cast(target_ty) => {
                    let val = eval_stack.pop().unwrap();
                    let casted = match target_ty {
                        crate::frontend::ElaichiType::I64 => DynValue::Int(val.to_i64()?),
                        crate::frontend::ElaichiType::F64 => DynValue::Float(val.to_f64()?),
                        crate::frontend::ElaichiType::Bool => DynValue::Bool(val.is_truthy()),
                        crate::frontend::ElaichiType::String => DynValue::Str(format!("{}", val)),
                        _ => val,
                    };
                    eval_stack.push(casted);
                    ip += 1;
                }
                OpCode::Not => {
                    let val = eval_stack.pop().unwrap();
                    eval_stack.push(DynValue::Bool(!val.is_truthy()));
                    ip += 1;
                }
                OpCode::Neg => {
                    let val = eval_stack.pop().unwrap();
                    match val {
                        DynValue::Int(n) => eval_stack.push(DynValue::Int(-n)),
                        DynValue::Float(f) => eval_stack.push(DynValue::Float(-f)),
                        other => return Err(format!("Cannot negate {:?}", other)),
                    }
                    ip += 1;
                }
                OpCode::Add => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => eval_stack.push(DynValue::Int(x + y)),
                        (DynValue::Float(x), DynValue::Float(y)) => eval_stack.push(DynValue::Float(x + y)),
                        (DynValue::Int(x), DynValue::Float(y)) => eval_stack.push(DynValue::Float(x as f64 + y)),
                        (DynValue::Float(x), DynValue::Int(y)) => eval_stack.push(DynValue::Float(x + y as f64)),
                        (DynValue::Str(x), DynValue::Str(y)) => eval_stack.push(DynValue::Str(format!("{}{}", x, y))),
                        (DynValue::Str(x), other) => eval_stack.push(DynValue::Str(format!("{}{}", x, other))),
                        (a, b) => return Err(format!("Cannot add dynamic values: {:?} + {:?}", a, b)),
                    }
                    ip += 1;
                }
                OpCode::Sub => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => eval_stack.push(DynValue::Int(x - y)),
                        (DynValue::Float(x), DynValue::Float(y)) => eval_stack.push(DynValue::Float(x - y)),
                        (a, b) => return Err(format!("Cannot subtract dynamic values: {:?} - {:?}", a, b)),
                    }
                    ip += 1;
                }
                OpCode::Mul => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => eval_stack.push(DynValue::Int(x * y)),
                        (DynValue::Float(x), DynValue::Float(y)) => eval_stack.push(DynValue::Float(x * y)),
                        (a, b) => return Err(format!("Cannot multiply dynamic values: {:?} * {:?}", a, b)),
                    }
                    ip += 1;
                }
                OpCode::Div => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => eval_stack.push(DynValue::Int(x / y)),
                        (DynValue::Float(x), DynValue::Float(y)) => eval_stack.push(DynValue::Float(x / y)),
                        (a, b) => return Err(format!("Cannot divide dynamic values: {:?} / {:?}", a, b)),
                    }
                    ip += 1;
                }
                OpCode::Mod => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => eval_stack.push(DynValue::Int(x % y)),
                        (a, b) => return Err(format!("Cannot mod dynamic values: {:?} % {:?}", a, b)),
                    }
                    ip += 1;
                }
                OpCode::Equal => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    eval_stack.push(DynValue::Bool(a == b));
                    ip += 1;
                }
                OpCode::NotEqual => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    eval_stack.push(DynValue::Bool(a != b));
                    ip += 1;
                }
                OpCode::Less => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    let res = match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => x < y,
                        (DynValue::Float(x), DynValue::Float(y)) => x < y,
                        _ => false,
                    };
                    eval_stack.push(DynValue::Bool(res));
                    ip += 1;
                }
                OpCode::LessEqual => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    let res = match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => x <= y,
                        (DynValue::Float(x), DynValue::Float(y)) => x <= y,
                        _ => false,
                    };
                    eval_stack.push(DynValue::Bool(res));
                    ip += 1;
                }
                OpCode::Greater => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    let res = match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => x > y,
                        (DynValue::Float(x), DynValue::Float(y)) => x > y,
                        _ => false,
                    };
                    eval_stack.push(DynValue::Bool(res));
                    ip += 1;
                }
                OpCode::GreaterEqual => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    let res = match (a, b) {
                        (DynValue::Int(x), DynValue::Int(y)) => x >= y,
                        (DynValue::Float(x), DynValue::Float(y)) => x >= y,
                        _ => false,
                    };
                    eval_stack.push(DynValue::Bool(res));
                    ip += 1;
                }
                OpCode::And => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    eval_stack.push(DynValue::Bool(a.is_truthy() && b.is_truthy()));
                    ip += 1;
                }
                OpCode::Or => {
                    let b = eval_stack.pop().unwrap();
                    let a = eval_stack.pop().unwrap();
                    eval_stack.push(DynValue::Bool(a.is_truthy() || b.is_truthy()));
                    ip += 1;
                }
                // First-class terminal operations
                OpCode::TerminalPrint => {
                    let val = eval_stack.pop().unwrap_or(DynValue::Nil);
                    print!("{}", val);
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                    ip += 1;
                }
                OpCode::TerminalPrintln => {
                    let val = eval_stack.pop().unwrap_or(DynValue::Nil);
                    println!("{}", val);
                    ip += 1;
                }
                OpCode::TerminalError => {
                    let val = eval_stack.pop().unwrap_or(DynValue::Nil);
                    eprintln!("[ERROR] {}", val);
                    ip += 1;
                }
                OpCode::TerminalWarn => {
                    let val = eval_stack.pop().unwrap_or(DynValue::Nil);
                    println!("[WARN] {}", val);
                    ip += 1;
                }
                OpCode::TerminalClear => {
                    // ANSI clear screen code
                    print!("\x1B[2J\x1B[1;1H");
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                    ip += 1;
                }
                OpCode::JumpIfFalse(target) => {
                    let cond = eval_stack.pop().unwrap_or(DynValue::Nil);
                    if !cond.is_truthy() {
                        ip = *target;
                    } else {
                        ip += 1;
                    }
                }
                OpCode::Jump(target) => {
                    ip = *target;
                }
                OpCode::Call(callee, arg_count) => {
                    let mut call_args = Vec::new();
                    for _ in 0..*arg_count {
                        call_args.push(eval_stack.pop().unwrap());
                    }
                    call_args.reverse();
                    let result = self.call_dynamic(callee, call_args)?;
                    eval_stack.push(result);
                    ip += 1;
                }
                OpCode::Return => {
                    return Ok(eval_stack.pop().unwrap_or(DynValue::Nil));
                }
            }
        }

        Ok(DynValue::Nil)
    }

    pub fn call_static(&mut self, name: &str, args: Vec<StaticValue>) -> Result<StaticValue, String> {
        let (_func_def, code) = match self.bifurcator.static_functions.get(name) {
            Some(pair) => (pair.0.clone(), pair.1.clone()),
            None => {
                if let Some((dyn_func_def, _)) = self.bifurcator.dynamic_functions.get(name).cloned() {
                    let dyn_args = args.iter().map(Bridge::static_to_dynamic).collect();
                    let dyn_res = self.call_dynamic(name, dyn_args)?;
                    return Bridge::dynamic_to_static(&dyn_res, &dyn_func_def.return_ty);
                }
                return Err(format!("Undefined static function '{}'", name));
            }
        };

        let mut locals = vec![StaticValue::I64(0); code.local_types.len()];
        for (i, arg) in args.into_iter().enumerate() {
            if i < locals.len() {
                locals[i] = arg;
            }
        }

        let mut stack: Vec<StaticValue> = Vec::new();
        let mut ip = 0;

        while ip < code.code.len() {
            match &code.code[ip] {
                StaticInst::ConstI64(n) => {
                    stack.push(StaticValue::I64(*n));
                    ip += 1;
                }
                StaticInst::ConstF64(f) => {
                    stack.push(StaticValue::F64(*f));
                    ip += 1;
                }
                StaticInst::ConstBool(b) => {
                    stack.push(StaticValue::Bool(*b));
                    ip += 1;
                }
                StaticInst::LoadLocal(idx) => {
                    stack.push(locals[*idx].clone());
                    ip += 1;
                }
                StaticInst::StoreLocal(idx) => {
                    let v = stack.pop().unwrap();
                    locals[*idx] = v;
                    ip += 1;
                }
                StaticInst::I64Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::I64(x + y));
                    }
                    ip += 1;
                }
                StaticInst::I64Sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::I64(x - y));
                    }
                    ip += 1;
                }
                StaticInst::I64Mul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::I64(x * y));
                    }
                    ip += 1;
                }
                StaticInst::I64Div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::I64(x / y));
                    }
                    ip += 1;
                }
                StaticInst::I64Mod => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::I64(x % y));
                    }
                    ip += 1;
                }
                StaticInst::F64Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::F64(x + y));
                    ip += 1;
                }
                StaticInst::F64Sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::F64(x - y));
                    ip += 1;
                }
                StaticInst::F64Mul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::F64(x * y));
                    ip += 1;
                }
                StaticInst::F64Div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::F64(x / y));
                    ip += 1;
                }
                StaticInst::F64Eq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x == y));
                    ip += 1;
                }
                StaticInst::F64Neq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x != y));
                    ip += 1;
                }
                StaticInst::F64Lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x < y));
                    ip += 1;
                }
                StaticInst::F64Le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x <= y));
                    ip += 1;
                }
                StaticInst::F64Gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x > y));
                    ip += 1;
                }
                StaticInst::F64Ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let x = a.to_f64()?;
                    let y = b.to_f64()?;
                    stack.push(StaticValue::Bool(x >= y));
                    ip += 1;
                }
                StaticInst::I64Eq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x == y));
                    }
                    ip += 1;
                }
                StaticInst::I64Neq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x != y));
                    }
                    ip += 1;
                }
                StaticInst::I64Lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x < y));
                    }
                    ip += 1;
                }
                StaticInst::I64Le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x <= y));
                    }
                    ip += 1;
                }
                StaticInst::I64Gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x > y));
                    }
                    ip += 1;
                }
                StaticInst::I64Ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    if let (StaticValue::I64(x), StaticValue::I64(y)) = (a, b) {
                        stack.push(StaticValue::Bool(x >= y));
                    }
                    ip += 1;
                }
                StaticInst::JumpIfFalse(target) => {
                    let cond = stack.pop().unwrap();
                    if let StaticValue::Bool(false) = cond {
                        ip = *target;
                    } else {
                        ip += 1;
                    }
                }
                StaticInst::Jump(target) => {
                    ip = *target;
                }
                StaticInst::Call(callee, arg_count) => {
                    let mut call_args = Vec::new();
                    for _ in 0..*arg_count {
                        call_args.push(stack.pop().unwrap());
                    }
                    call_args.reverse();
                    let res = self.call_static(callee, call_args)?;
                    stack.push(res);
                    ip += 1;
                }
                StaticInst::Return => {
                    return Ok(stack.pop().unwrap_or(StaticValue::I64(0)));
                }
            }
        }

        Ok(StaticValue::I64(0))
    }

    fn call_stdlib(&mut self, name: &str, args: &[DynValue]) -> Result<Option<DynValue>, String> {
        let bifurcator = &self.bifurcator;
        let dyn_globals = &self.dyn_globals;
        super::stdlib::StdLib::dispatch(name, args, |callee, call_args| {
            let mut sub_rt = Runtime {
                bifurcator: bifurcator.clone(),
                dyn_globals: dyn_globals.clone(),
            };
            sub_rt.call_dynamic(callee, call_args)
        })
    }
}
