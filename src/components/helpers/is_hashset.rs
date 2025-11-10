use super::import::*;

pub fn is_hashset(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty
        && let Some(segment) = path.segments.last()
    {
        return segment.ident == "HashSet";
    }
    false
}
