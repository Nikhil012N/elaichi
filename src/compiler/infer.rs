use super::types::{Substitution, Type, TypeEnv, TypeScheme};
use crate::frontend::ast::{BinaryOp, Expr, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UnificationMismatch(Type, Type),
    OccursCheckFailed(usize, Type),
    UnboundVariable(String),
    NotAFunction(Type),
    ArityMismatch { expected: usize, got: usize },
    InvalidRecordAccess(String),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnificationMismatch(t1, t2) => {
                write!(f, "Type mismatch error: expected {}, but found {}", t1, t2)
            }
            TypeError::OccursCheckFailed(var, ty) => {
                write!(f, "Occurs check failed: cyclic type 't{} in {}", var, ty)
            }
            TypeError::UnboundVariable(var) => {
                write!(f, "Unbound identifier '{}' not present in type environment", var)
            }
            TypeError::NotAFunction(ty) => {
                write!(f, "Expression of type {} is not callable as a function", ty)
            }
            TypeError::ArityMismatch { expected, got } => {
                write!(f, "Function call expected {} arguments, but got {}", expected, got)
            }
            TypeError::InvalidRecordAccess(field) => {
                write!(f, "Field '{}' does not exist on record", field)
            }
        }
    }
}

pub struct TypeInference {
    next_var_id: usize,
}

impl TypeInference {
    pub fn new() -> Self {
        Self { next_var_id: 0 }
    }

    pub fn fresh_var(&mut self) -> Type {
        let id = self.next_var_id;
        self.next_var_id += 1;
        Type::Var(id)
    }

    pub fn instantiate(&mut self, scheme: &TypeScheme) -> Type {
        let mut sub = Substitution::new();
        for &v in &scheme.vars {
            sub.insert(v, self.fresh_var());
        }
        sub.apply(&scheme.ty)
    }

    pub fn generalize(&self, env: &TypeEnv, ty: &Type) -> TypeScheme {
        let env_free = env.free_vars();
        let ty_free = ty.free_vars();
        let vars: Vec<usize> = ty_free.difference(&env_free).copied().collect();
        TypeScheme {
            vars,
            ty: ty.clone(),
        }
    }

    pub fn unify(&self, t1: &Type, t2: &Type) -> Result<Substitution, TypeError> {
        match (t1, t2) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::Bool, Type::Bool)
            | (Type::Str, Type::Str)
            | (Type::Void, Type::Void) => Ok(Substitution::new()),

            (Type::Var(id1), Type::Var(id2)) if id1 == id2 => Ok(Substitution::new()),

            (Type::Var(id), ty) | (ty, Type::Var(id)) => self.bind_var(*id, ty),

            (Type::List(inner1), Type::List(inner2)) => self.unify(inner1, inner2),

            (Type::Fn(params1, ret1), Type::Fn(params2, ret2)) => {
                if params1.len() != params2.len() {
                    return Err(TypeError::ArityMismatch {
                        expected: params1.len(),
                        got: params2.len(),
                    });
                }
                let mut current_sub = Substitution::new();
                for (p1, p2) in params1.iter().zip(params2.iter()) {
                    let s = self.unify(&current_sub.apply(p1), &current_sub.apply(p2))?;
                    current_sub = s.compose(&current_sub);
                }
                let ret_sub = self.unify(&current_sub.apply(ret1), &current_sub.apply(ret2))?;
                Ok(ret_sub.compose(&current_sub))
            }

            (t1, t2) => Err(TypeError::UnificationMismatch(t1.clone(), t2.clone())),
        }
    }

    fn bind_var(&self, var: usize, ty: &Type) -> Result<Substitution, TypeError> {
        if let Type::Var(id) = ty {
            if *id == var {
                return Ok(Substitution::new());
            }
        }
        if ty.free_vars().contains(&var) {
            return Err(TypeError::OccursCheckFailed(var, ty.clone()));
        }
        let mut sub = Substitution::new();
        sub.insert(var, ty.clone());
        Ok(sub)
    }

    /// Algorithm W: Main expression type inference
    pub fn infer_expr(
        &mut self,
        env: &TypeEnv,
        expr: &Expr,
    ) -> Result<(Substitution, Type), TypeError> {
        match expr {
            Expr::IntLit(_) => Ok((Substitution::new(), Type::Int)),
            Expr::FloatLit(_) => Ok((Substitution::new(), Type::Float)),
            Expr::BoolLit(_) => Ok((Substitution::new(), Type::Bool)),
            Expr::StringLit(_) => Ok((Substitution::new(), Type::Str)),

            Expr::Var(name) => match env.get(name) {
                Some(scheme) => Ok((Substitution::new(), self.instantiate(scheme))),
                None => Err(TypeError::UnboundVariable(name.clone())),
            },

            Expr::Unary { op, expr } => {
                let (sub, expr_ty) = self.infer_expr(env, expr)?;
                match op {
                    UnaryOp::Not => {
                        let unify_sub = self.unify(&expr_ty, &Type::Bool)?;
                        Ok((unify_sub.compose(&sub), Type::Bool))
                    }
                    UnaryOp::Neg => {
                        let unify_sub = self.unify(&expr_ty, &Type::Int)?;
                        Ok((unify_sub.compose(&sub), Type::Int))
                    }
                }
            }

            Expr::Binary { op, left, right } => {
                let (s1, t1) = self.infer_expr(env, left)?;
                let (s2, t2) = self.infer_expr(&env.apply(&s1), right)?;
                let composed_sub = s2.compose(&s1);

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        let unify_left = self.unify(&composed_sub.apply(&t1), &Type::Int)?;
                        let left_composed = unify_left.compose(&composed_sub);
                        let unify_right = self.unify(&left_composed.apply(&t2), &Type::Int)?;
                        Ok((unify_right.compose(&left_composed), Type::Int))
                    }
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        let unify_sub = self.unify(&composed_sub.apply(&t1), &composed_sub.apply(&t2))?;
                        Ok((unify_sub.compose(&composed_sub), Type::Bool))
                    }
                    BinaryOp::Less
                    | BinaryOp::LessEqual
                    | BinaryOp::Greater
                    | BinaryOp::GreaterEqual => {
                        let unify_left = self.unify(&composed_sub.apply(&t1), &Type::Int)?;
                        let left_composed = unify_left.compose(&composed_sub);
                        let unify_right = self.unify(&left_composed.apply(&t2), &Type::Int)?;
                        Ok((unify_right.compose(&left_composed), Type::Bool))
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        let unify_left = self.unify(&composed_sub.apply(&t1), &Type::Bool)?;
                        let left_composed = unify_left.compose(&composed_sub);
                        let unify_right = self.unify(&left_composed.apply(&t2), &Type::Bool)?;
                        Ok((unify_right.compose(&left_composed), Type::Bool))
                    }
                }
            }

            Expr::ListLit(elements) => {
                if elements.is_empty() {
                    let elem_var = self.fresh_var();
                    return Ok((Substitution::new(), Type::List(Box::new(elem_var))));
                }

                let (s1, first_ty) = self.infer_expr(env, &elements[0])?;
                let mut current_sub = s1;
                let mut target_elem_ty = first_ty;

                for elem in &elements[1..] {
                    let (s_elem, elem_ty) = self.infer_expr(&env.apply(&current_sub), elem)?;
                    current_sub = s_elem.compose(&current_sub);
                    let unify_sub = self.unify(&current_sub.apply(&target_elem_ty), &elem_ty)?;
                    current_sub = unify_sub.compose(&current_sub);
                    target_elem_ty = current_sub.apply(&target_elem_ty);
                }

                Ok((current_sub, Type::List(Box::new(target_elem_ty))))
            }

            Expr::Call { callee, args } => {
                let callee_scheme = match env.get(callee) {
                    Some(s) => s.clone(),
                    None => return Err(TypeError::UnboundVariable(callee.clone())),
                };
                let callee_ty = self.instantiate(&callee_scheme);
                let result_ty = self.fresh_var();

                let mut current_sub = Substitution::new();
                let mut arg_types = Vec::new();

                for arg in args {
                    let (s_arg, arg_ty) = self.infer_expr(&env.apply(&current_sub), arg)?;
                    current_sub = s_arg.compose(&current_sub);
                    arg_types.push(arg_ty);
                }

                let expected_fn_ty = Type::Fn(arg_types, Box::new(result_ty.clone()));
                let unify_sub = self.unify(&current_sub.apply(&callee_ty), &expected_fn_ty)?;
                let final_sub = unify_sub.compose(&current_sub);

                Ok((final_sub.clone(), final_sub.apply(&result_ty)))
            }

            Expr::AnonymousFn { params, body } => {
                let mut local_env = env.clone();
                let mut param_types = Vec::new();

                for p in params {
                    let v = self.fresh_var();
                    local_env.insert(p.clone(), TypeScheme::monomorphic(v.clone()));
                    param_types.push(v);
                }

                let (body_sub, body_ty) = self.infer_expr(&local_env, body)?;
                let inferred_params: Vec<Type> =
                    param_types.iter().map(|p| body_sub.apply(p)).collect();

                Ok((
                    body_sub,
                    Type::Fn(inferred_params, Box::new(body_ty)),
                ))
            }

            _ => Ok((Substitution::new(), Type::Void)),
        }
    }
}
