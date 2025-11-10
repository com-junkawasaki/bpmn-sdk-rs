//! Task assignment policies.

/// Assignment policy
#[derive(Debug, Clone)]
pub enum AssignmentPolicy {
    /// Assign to specific user
    User(String),
    /// Assign to candidate users
    CandidateUsers(Vec<String>),
    /// Assign to candidate groups
    CandidateGroups(Vec<String>),
    /// Expression-based assignment
    Expression(String),
}

