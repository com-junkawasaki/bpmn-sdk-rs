//! Process execution monitor.

/// Process monitor
pub struct ProcessMonitor;

impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

