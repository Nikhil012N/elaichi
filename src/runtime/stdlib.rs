use crate::compiler::DynValue;

pub struct StdLib;

impl StdLib {
    pub fn dispatch<F>(name: &str, args: &[DynValue], mut call_dynamic_cb: F) -> Result<Option<DynValue>, String>
    where
        F: FnMut(&str, Vec<DynValue>) -> Result<DynValue, String>,
    {
        match name {
            // Collection inspection
            "len" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::List(l) => Ok(Some(DynValue::Int(l.len() as i64))),
                        DynValue::Str(s) => Ok(Some(DynValue::Int(s.len() as i64))),
                        DynValue::Map(m) => Ok(Some(DynValue::Int(m.len() as i64))),
                        _ => Err("len() requires a list, string, or map".to_string()),
                    }
                } else {
                    Err("len() takes 1 argument".to_string())
                }
            }
            "push" => {
                if args.len() == 2 {
                    if let DynValue::List(ref l) = args[0] {
                        let mut new_l = l.clone();
                        new_l.push(args[1].clone());
                        Ok(Some(DynValue::List(new_l)))
                    } else {
                        Err("push() requires a list as first argument".to_string())
                    }
                } else {
                    Err("push() takes 2 arguments".to_string())
                }
            }
            "str" => {
                if let Some(arg) = args.first() {
                    Ok(Some(DynValue::Str(format!("{}", arg))))
                } else {
                    Err("str() takes 1 argument".to_string())
                }
            }
            "keys" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Map(m) => {
                            let mut k_list: Vec<DynValue> = m.keys().cloned().map(DynValue::Str).collect();
                            k_list.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
                            Ok(Some(DynValue::List(k_list)))
                        }
                        _ => Err("keys() requires a map".to_string()),
                    }
                } else {
                    Err("keys() takes 1 argument".to_string())
                }
            }
            "values" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Map(m) => {
                            let mut keys: Vec<String> = m.keys().cloned().collect();
                            keys.sort();
                            let v_list: Vec<DynValue> = keys.iter().map(|k| m[k].clone()).collect();
                            Ok(Some(DynValue::List(v_list)))
                        }
                        _ => Err("values() requires a map".to_string()),
                    }
                } else {
                    Err("values() takes 1 argument".to_string())
                }
            }
            "contains" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (DynValue::List(l), item) => Ok(Some(DynValue::Bool(l.contains(item)))),
                        (DynValue::Map(m), DynValue::Str(k)) => Ok(Some(DynValue::Bool(m.contains_key(k)))),
                        (DynValue::Str(s), DynValue::Str(sub)) => Ok(Some(DynValue::Bool(s.contains(sub)))),
                        _ => Err("contains() requires (list, item), (map, key), or (str, substr)".to_string()),
                    }
                } else {
                    Err("contains() takes 2 arguments".to_string())
                }
            }

            // Math Primitives
            "abs" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Int(n) => Ok(Some(DynValue::Int(n.abs()))),
                        DynValue::Float(f) => Ok(Some(DynValue::Float(f.abs()))),
                        _ => Err("abs() requires a number".to_string()),
                    }
                } else {
                    Err("abs() takes 1 argument".to_string())
                }
            }
            "min" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (DynValue::Int(a), DynValue::Int(b)) => Ok(Some(DynValue::Int(*a.min(b)))),
                        (DynValue::Float(a), DynValue::Float(b)) => Ok(Some(DynValue::Float(a.min(*b)))),
                        _ => Err("min() requires two matching numeric arguments".to_string()),
                    }
                } else {
                    Err("min() takes 2 arguments".to_string())
                }
            }
            "max" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (DynValue::Int(a), DynValue::Int(b)) => Ok(Some(DynValue::Int(*a.max(b)))),
                        (DynValue::Float(a), DynValue::Float(b)) => Ok(Some(DynValue::Float(a.max(*b)))),
                        _ => Err("max() requires two matching numeric arguments".to_string()),
                    }
                } else {
                    Err("max() takes 2 arguments".to_string())
                }
            }
            "sqrt" => {
                if let Some(arg) = args.first() {
                    let val = arg.to_f64()?;
                    if val < 0.0 {
                        return Err("sqrt() requires non-negative value".to_string());
                    }
                    Ok(Some(DynValue::Float(val.sqrt())))
                } else {
                    Err("sqrt() takes 1 argument".to_string())
                }
            }
            "floor" => {
                if let Some(arg) = args.first() {
                    let val = arg.to_f64()?;
                    Ok(Some(DynValue::Int(val.floor() as i64)))
                } else {
                    Err("floor() takes 1 argument".to_string())
                }
            }
            "ceil" => {
                if let Some(arg) = args.first() {
                    let val = arg.to_f64()?;
                    Ok(Some(DynValue::Int(val.ceil() as i64)))
                } else {
                    Err("ceil() takes 1 argument".to_string())
                }
            }
            "round" => {
                if let Some(arg) = args.first() {
                    let val = arg.to_f64()?;
                    Ok(Some(DynValue::Int(val.round() as i64)))
                } else {
                    Err("round() takes 1 argument".to_string())
                }
            }
            "pow" => {
                if args.len() == 2 {
                    let base = args[0].to_f64()?;
                    let exponent = args[1].to_f64()?;
                    Ok(Some(DynValue::Float(base.powf(exponent))))
                } else {
                    Err("pow() takes 2 arguments: (base, exponent)".to_string())
                }
            }
            "clamp" => {
                if args.len() == 3 {
                    let val = args[0].to_f64()?;
                    let min_val = args[1].to_f64()?;
                    let max_val = args[2].to_f64()?;
                    let clamped = val.max(min_val).min(max_val);
                    if let (DynValue::Int(_), DynValue::Int(_), DynValue::Int(_)) = (&args[0], &args[1], &args[2]) {
                        Ok(Some(DynValue::Int(clamped as i64)))
                    } else {
                        Ok(Some(DynValue::Float(clamped)))
                    }
                } else {
                    Err("clamp() takes 3 arguments: (val, min, max)".to_string())
                }
            }

            // String and Collection Transformers
            "type_of" | "typeOf" => {
                if let Some(arg) = args.first() {
                    let t = match arg {
                        DynValue::Nil => "nil",
                        DynValue::Bool(_) => "bool",
                        DynValue::Int(_) => "int",
                        DynValue::Float(_) => "float",
                        DynValue::Str(_) => "string",
                        DynValue::List(_) => "list",
                        DynValue::Map(_) => "map",
                    };
                    Ok(Some(DynValue::Str(t.to_string())))
                } else {
                    Err("typeOf() takes 1 argument".to_string())
                }
            }
            "slice" => {
                if args.len() >= 3 {
                    let start = args[1].to_i64()? as usize;
                    let end = args[2].to_i64()? as usize;
                    match &args[0] {
                        DynValue::List(l) => {
                            let s = start.min(l.len());
                            let e = end.min(l.len()).max(s);
                            Ok(Some(DynValue::List(l[s..e].to_vec())))
                        }
                        DynValue::Str(st) => {
                            let s = start.min(st.len());
                            let e = end.min(st.len()).max(s);
                            Ok(Some(DynValue::Str(st[s..e].to_string())))
                        }
                        _ => Err("slice() requires a list or string as first argument".to_string()),
                    }
                } else {
                    Err("slice() takes 3 arguments: (collection, start, end)".to_string())
                }
            }
            "split" => {
                if args.len() == 2 {
                    if let (DynValue::Str(s), DynValue::Str(pat)) = (&args[0], &args[1]) {
                        let parts: Vec<DynValue> = s.split(pat).map(|sub| DynValue::Str(sub.to_string())).collect();
                        Ok(Some(DynValue::List(parts)))
                    } else {
                        Err("split() requires two strings: (source, pattern)".to_string())
                    }
                } else {
                    Err("split() takes 2 arguments".to_string())
                }
            }
            "join" => {
                if args.len() == 2 {
                    if let (DynValue::List(l), DynValue::Str(sep)) = (&args[0], &args[1]) {
                        let strings: Vec<String> = l.iter().map(|v| format!("{}", v)).collect();
                        Ok(Some(DynValue::Str(strings.join(sep))))
                    } else {
                        Err("join() requires (list, separator_string)".to_string())
                    }
                } else {
                    Err("join() takes 2 arguments".to_string())
                }
            }
            "reverse" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::List(l) => {
                            let mut rev = l.clone();
                            rev.reverse();
                            Ok(Some(DynValue::List(rev)))
                        }
                        DynValue::Str(s) => {
                            let rev: String = s.chars().rev().collect();
                            Ok(Some(DynValue::Str(rev)))
                        }
                        _ => Err("reverse() requires a list or string".to_string()),
                    }
                } else {
                    Err("reverse() takes 1 argument".to_string())
                }
            }
            "trim" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Str(s) => Ok(Some(DynValue::Str(s.trim().to_string()))),
                        _ => Err("trim() requires a string".to_string()),
                    }
                } else {
                    Err("trim() takes 1 argument".to_string())
                }
            }
            "to_upper" | "toUpper" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Str(s) => Ok(Some(DynValue::Str(s.to_uppercase()))),
                        _ => Err("toUpper() requires a string".to_string()),
                    }
                } else {
                    Err("toUpper() takes 1 argument".to_string())
                }
            }
            "to_lower" | "toLower" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::Str(s) => Ok(Some(DynValue::Str(s.to_lowercase()))),
                        _ => Err("toLower() requires a string".to_string()),
                    }
                } else {
                    Err("toLower() takes 1 argument".to_string())
                }
            }
            "starts_with" | "startsWith" => {
                if args.len() == 2 {
                    if let (DynValue::Str(s), DynValue::Str(prefix)) = (&args[0], &args[1]) {
                        Ok(Some(DynValue::Bool(s.starts_with(prefix))))
                    } else {
                        Err("startsWith() requires two strings".to_string())
                    }
                } else {
                    Err("startsWith() takes 2 arguments".to_string())
                }
            }
            "ends_with" | "endsWith" => {
                if args.len() == 2 {
                    if let (DynValue::Str(s), DynValue::Str(suffix)) = (&args[0], &args[1]) {
                        Ok(Some(DynValue::Bool(s.ends_with(suffix))))
                    } else {
                        Err("endsWith() requires two strings".to_string())
                    }
                } else {
                    Err("endsWith() takes 2 arguments".to_string())
                }
            }
            "replace" => {
                if args.len() == 3 {
                    if let (DynValue::Str(s), DynValue::Str(from), DynValue::Str(to)) = (&args[0], &args[1], &args[2]) {
                        Ok(Some(DynValue::Str(s.replace(from, to))))
                    } else {
                        Err("replace() requires three strings: (source, from, to)".to_string())
                    }
                } else {
                    Err("replace() takes 3 arguments".to_string())
                }
            }
            "sort" => {
                if let Some(arg) = args.first() {
                    match arg {
                        DynValue::List(l) => {
                            let mut sorted = l.clone();
                            sorted.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
                            Ok(Some(DynValue::List(sorted)))
                        }
                        _ => Err("sort() requires a list".to_string()),
                    }
                } else {
                    Err("sort() takes 1 argument".to_string())
                }
            }
            "time_now" | "timeNow" => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0);
                Ok(Some(DynValue::Int(now)))
            }

            // Functional Higher-Order Collections
            "filter" => {
                if args.len() >= 2 {
                    if let DynValue::List(l) = &args[0] {
                        match &args[1] {
                            DynValue::Str(predicate_fn_name) => {
                                let mut filtered = Vec::new();
                                for item in l {
                                    let res = call_dynamic_cb(predicate_fn_name, vec![item.clone()])?;
                                    if res.is_truthy() {
                                        filtered.push(item.clone());
                                    }
                                }
                                Ok(Some(DynValue::List(filtered)))
                            }
                            _ => Err("filter() requires (list, functionNameString)".to_string()),
                        }
                    } else {
                        Err("filter() requires a list as first argument".to_string())
                    }
                } else {
                    Err("filter() takes 2 arguments".to_string())
                }
            }
            "mapList" | "map_list" => {
                if args.len() >= 2 {
                    if let DynValue::List(l) = &args[0] {
                        match &args[1] {
                            DynValue::Str(transform_fn_name) => {
                                let mut mapped = Vec::new();
                                for item in l {
                                    let res = call_dynamic_cb(transform_fn_name, vec![item.clone()])?;
                                    mapped.push(res);
                                }
                                Ok(Some(DynValue::List(mapped)))
                            }
                            _ => Err("mapList() requires (list, functionNameString)".to_string()),
                        }
                    } else {
                        Err("mapList() requires a list as first argument".to_string())
                    }
                } else {
                    Err("mapList() takes 2 arguments".to_string())
                }
            }
            "reduce" => {
                if args.len() >= 3 {
                    if let DynValue::List(l) = &args[0] {
                        match &args[1] {
                            DynValue::Str(reducer_fn_name) => {
                                let mut acc = args[2].clone();
                                for item in l {
                                    acc = call_dynamic_cb(reducer_fn_name, vec![acc, item.clone()])?;
                                }
                                Ok(Some(acc))
                            }
                            _ => Err("reduce() requires (list, functionNameString, initialAcc)".to_string()),
                        }
                    } else {
                        Err("reduce() requires a list as first argument".to_string())
                    }
                } else {
                    Err("reduce() takes 3 arguments".to_string())
                }
            }
            "find" => {
                if args.len() >= 2 {
                    if let DynValue::List(l) = &args[0] {
                        match &args[1] {
                            DynValue::Str(predicate_fn_name) => {
                                for item in l {
                                    let res = call_dynamic_cb(predicate_fn_name, vec![item.clone()])?;
                                    if res.is_truthy() {
                                        return Ok(Some(item.clone()));
                                    }
                                }
                                Ok(Some(DynValue::Nil))
                            }
                            _ => Err("find() requires (list, functionNameString)".to_string()),
                        }
                    } else {
                        Err("find() requires a list as first argument".to_string())
                    }
                } else {
                    Err("find() takes 2 arguments".to_string())
                }
            }
            _ => Ok(None),
        }
    }
}
