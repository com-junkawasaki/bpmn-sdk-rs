//! Escalation management.

/// Escalation rule
#[derive(Debug, Clone)]
pub struct EscalationRule {
    pub after_duration: u64,
    pub action: EscalationAction,
}

/// Escalation action
#[derive(Debug, Clone)]
pub enum EscalationAction {
    Notify(String),
    Reassign(String),
    EscalateTo(String),
}

