use super::import::*;

pub fn is_function(ty: &Type) -> bool {
    matches!(ty, &Type::BareFn(_))
}
