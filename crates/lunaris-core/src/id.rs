//! Unique identifier types for engine resources

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// A unique identifier for engine resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(u64);

impl Id {
    /// Generate a new unique ID
    #[must_use]
    pub fn new() -> Self {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// Create an ID from a raw value (use with caution)
    #[must_use]
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }

    /// Get the raw value
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// Null/invalid ID
    pub const NULL: Self = Self(0);
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

/// A typed identifier wrapper for type-safe resource references
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypedId<T> {
    id: Id,
    _marker: std::marker::PhantomData<T>,
}

impl<T> TypedId<T> {
    /// Create a new typed ID
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: Id::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Get the underlying ID
    #[must_use]
    pub const fn id(self) -> Id {
        self.id
    }
}

impl<T> Default for TypedId<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids() {
        let id1 = Id::new();
        let id2 = Id::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn typed_ids_are_unique() {
        struct EntityMarker;
        let id1: TypedId<EntityMarker> = TypedId::new();
        let id2: TypedId<EntityMarker> = TypedId::new();
        assert_ne!(id1.id(), id2.id());
    }
}
