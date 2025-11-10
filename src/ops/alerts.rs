//! Alerting support.

/// Alert rule
#[derive(Debug, Clone)]
pub struct AlertRule {
    pub condition: String,
    pub severity: AlertSeverity,
    pub action: AlertAction,
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Alert action
#[derive(Debug, Clone)]
pub enum AlertAction {
    Notify(String),
    Escalate(String),
    Execute(String),
}

