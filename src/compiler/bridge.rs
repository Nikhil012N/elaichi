use crate::frontend::ElaichiType;
use super::static_engine::StaticValue;
use super::vm::DynValue;

pub struct Bridge;

impl Bridge {
    /// Unbox dynamic tagged value into raw static type (dynamic -> static)
    pub fn dynamic_to_static(dyn_val: &DynValue, target_ty: &ElaichiType) -> Result<StaticValue, String> {
        match target_ty {
            ElaichiType::I64 => {
                let n = dyn_val.to_i64()?;
                Ok(StaticValue::I64(n))
            }
            ElaichiType::F64 => {
                let f = dyn_val.to_f64()?;
                Ok(StaticValue::F64(f))
            }
            ElaichiType::Bool => match dyn_val {
                DynValue::Bool(b) => Ok(StaticValue::Bool(*b)),
                _ => Err(format!("Type guard error: expected bool, found {:?}", dyn_val)),
            },
            _ => Err(format!("Bridge cannot coerce {:?} to {:?}", dyn_val, target_ty)),
        }
    }

    /// Box raw static unboxed value into dynamic tagged value (static -> dynamic)
    pub fn static_to_dynamic(static_val: &StaticValue) -> DynValue {
        match static_val {
            StaticValue::I64(n) => DynValue::Int(*n),
            StaticValue::F64(f) => DynValue::Float(*f),
            StaticValue::Bool(b) => DynValue::Bool(*b),
            StaticValue::StructInstance { fields, .. } => {
                let dyn_fields = fields.iter().map(Self::static_to_dynamic).collect();
                DynValue::List(dyn_fields)
            }
        }
    }
}
