//! Type-safe identifiers and phantom types for validation state tracking.


/// Process identifier (newtype pattern for type safety)
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessId(pub String);

impl ProcessId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ProcessId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ProcessId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Element identifier (newtype pattern for type safety)
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ElementId(pub String);

impl ElementId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ElementId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ElementId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Activity identifier (newtype pattern for type safety)
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ActivityId(pub String);

impl ActivityId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for ActivityId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ActivityId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Marker trait for validation states
pub trait ValidationState: std::marker::Sized {}

/// Unvalidated state - IR has been constructed but not yet validated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unvalidated;

impl ValidationState for Unvalidated {}

/// Validated state - IR has passed static validation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validated;

impl ValidationState for Validated {}

/// Compiled state - IR has been compiled to ProcessDefinition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compiled;

impl ValidationState for Compiled {}

