//! SLA (Service Level Agreement) management.

/// SLA definition
#[derive(Debug, Clone)]
pub struct SLA {
    pub duration: u64,
    pub unit: SLAUnit,
}

/// SLA time unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SLAUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

