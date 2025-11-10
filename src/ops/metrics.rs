//! Metrics collection.

/// Process metrics
#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub process_count: u64,
    pub active_instances: u64,
    pub completed_instances: u64,
    pub failed_instances: u64,
}

