//! Static validation for BPMN process definitions.

pub mod validator;
pub mod rules;
pub mod reachability;

pub use validator::{Validator, ValidationResult, ValidationError, ValidationSeverity};
pub use rules::ValidationRule;

