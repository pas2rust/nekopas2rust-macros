use super::import::*;

pub fn is_hashmap(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty
        && let Some(segment) = path.segments.last()
    {
        return segment.ident == "HashMap";
    }
    false
}
