use std::collections::{HashMap, HashSet};
use std::fmt;

/// Type representation for Hindley-Milner Type Inference
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Primitive 64-bit signed integer
    Int,
    /// Primitive 64-bit IEEE-754 float
    Float,
    /// Boolean logic primitive
    Bool,
    /// UTF-8 string primitive
    Str,
    /// Unit / void type
    Void,
    /// Homogeneous list type
    List(Box<Type>),
    /// Record / map type with field keys and value types
    Record(Vec<(String, Type)>),
    /// Function type: (ParamTypes...) -> ReturnType
    Fn(Vec<Type>, Box<Type>),
    /// Type variable for polymorphic inference (e.g. 'a, 'b)
    Var(usize),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "i64"),
            Type::Float => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "str"),
            Type::Void => write!(f, "void"),
            Type::List(inner) => write!(f, "[{}]", inner),
            Type::Record(fields) => {
                let formatted: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{ {} }}", formatted.join(", "))
            }
            Type::Fn(params, ret) => {
                let formatted: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                write!(f, "({}) -> {}", formatted.join(", "), ret)
            }
            Type::Var(id) => write!(f, "'t{}", id),
        }
    }
}

/// Type scheme representing universally quantified types (∀ 'a. T)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeScheme {
    pub vars: Vec<usize>,
    pub ty: Type,
}

impl TypeScheme {
    pub fn monomorphic(ty: Type) -> Self {
        Self {
            vars: Vec::new(),
            ty,
        }
    }

    pub fn free_vars(&self) -> HashSet<usize> {
        let mut fv = self.ty.free_vars();
        for v in &self.vars {
            fv.remove(v);
        }
        fv
    }
}

impl Type {
    pub fn free_vars(&self) -> HashSet<usize> {
        let mut vars = HashSet::new();
        match self {
            Type::Var(id) => {
                vars.insert(*id);
            }
            Type::List(inner) => {
                vars.extend(inner.free_vars());
            }
            Type::Record(fields) => {
                for (_, ty) in fields {
                    vars.extend(ty.free_vars());
                }
            }
            Type::Fn(params, ret) => {
                for p in params {
                    vars.extend(p.free_vars());
                }
                vars.extend(ret.free_vars());
            }
            _ => {}
        }
        vars
    }
}

/// Substitution mapping type variable IDs to resolved Types
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    pub mappings: HashMap<usize, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, var: usize, ty: Type) {
        self.mappings.insert(var, ty);
    }

    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(target) = self.mappings.get(id) {
                    self.apply(target)
                } else {
                    Type::Var(*id)
                }
            }
            Type::List(inner) => Type::List(Box::new(self.apply(inner))),
            Type::Record(fields) => Type::Record(
                fields
                    .iter()
                    .map(|(k, v)| (k.clone(), self.apply(v)))
                    .collect(),
            ),
            Type::Fn(params, ret) => Type::Fn(
                params.iter().map(|p| self.apply(p)).collect(),
                Box::new(self.apply(ret)),
            ),
            primitive => primitive.clone(),
        }
    }

    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = HashMap::new();
        for (var, ty) in &other.mappings {
            result.insert(*var, self.apply(ty));
        }
        for (var, ty) in &self.mappings {
            result.insert(*var, ty.clone());
        }
        Substitution { mappings: result }
    }
}

/// Type Environment mapping identifier names to universal Type Schemes
#[derive(Debug, Clone, Default)]
pub struct TypeEnv {
    bindings: HashMap<String, TypeScheme>,
}

impl TypeEnv {
    pub fn new() -> Self {
        let mut env = Self {
            bindings: HashMap::new(),
        };
        env.seed_primitives();
        env
    }

    pub fn insert(&mut self, name: String, scheme: TypeScheme) {
        self.bindings.insert(name, scheme);
    }

    pub fn get(&self, name: &str) -> Option<&TypeScheme> {
        self.bindings.get(name)
    }

    pub fn free_vars(&self) -> HashSet<usize> {
        let mut fv = HashSet::new();
        for scheme in self.bindings.values() {
            fv.extend(scheme.free_vars());
        }
        fv
    }

    pub fn apply(&self, sub: &Substitution) -> TypeEnv {
        let mut new_bindings = HashMap::new();
        for (name, scheme) in &self.bindings {
            let new_scheme = TypeScheme {
                vars: scheme.vars.clone(),
                ty: sub.apply(&scheme.ty),
            };
            new_bindings.insert(name.clone(), new_scheme);
        }
        TypeEnv {
            bindings: new_bindings,
        }
    }

    fn seed_primitives(&mut self) {
        // Builtin math primitives
        self.insert(
            "sqrt".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![Type::Float], Box::new(Type::Float))),
        );
        self.insert(
            "pow".to_string(),
            TypeScheme::monomorphic(Type::Fn(
                vec![Type::Float, Type::Float],
                Box::new(Type::Float),
            )),
        );
        self.insert(
            "round".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![Type::Float], Box::new(Type::Int))),
        );
        self.insert(
            "timeNow".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![], Box::new(Type::Int))),
        );

        // Builtin string operations
        self.insert(
            "toUpper".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![Type::Str], Box::new(Type::Str))),
        );
        self.insert(
            "toLower".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![Type::Str], Box::new(Type::Str))),
        );
        self.insert(
            "trim".to_string(),
            TypeScheme::monomorphic(Type::Fn(vec![Type::Str], Box::new(Type::Str))),
        );
        self.insert(
            "startsWith".to_string(),
            TypeScheme::monomorphic(Type::Fn(
                vec![Type::Str, Type::Str],
                Box::new(Type::Bool),
            )),
        );
        self.insert(
            "endsWith".to_string(),
            TypeScheme::monomorphic(Type::Fn(
                vec![Type::Str, Type::Str],
                Box::new(Type::Bool),
            )),
        );
        self.insert(
            "split".to_string(),
            TypeScheme::monomorphic(Type::Fn(
                vec![Type::Str, Type::Str],
                Box::new(Type::List(Box::new(Type::Str))),
            )),
        );
        self.insert(
            "join".to_string(),
            TypeScheme::monomorphic(Type::Fn(
                vec![Type::List(Box::new(Type::Str)), Type::Str],
                Box::new(Type::Str),
            )),
        );
    }
}
