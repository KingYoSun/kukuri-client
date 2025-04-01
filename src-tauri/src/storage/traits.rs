//! Common traits for data models stored in the repository.

/// Trait for models that have a unique identifier string.
pub trait HasId {
    fn id(&self) -> &str;
}

/// Trait specifically for Post-like models.
// Renamed from `Post` to avoid conflict with the struct name.
pub trait PostEntry {
    fn author_id(&self) -> &str;
    fn content(&self) -> &str;
    fn created_at(&self) -> i64;
}
