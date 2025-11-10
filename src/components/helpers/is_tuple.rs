use super::import::*;

pub fn is_tuple(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(_))
}
