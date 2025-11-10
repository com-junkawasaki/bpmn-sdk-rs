//! Human task manager.

/// Task manager for human tasks
pub struct TaskManager;

impl TaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

