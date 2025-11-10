//! Tracing support.

/// Trace event
#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub process_id: String,
    pub element_id: Option<String>,
    pub data: Option<String>,
}

