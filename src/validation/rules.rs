//! Validation rules as separate functions.

use crate::core::BpmnIR;
use crate::core::Unvalidated;
use crate::validation::ValidationError;

/// Validation rule trait
pub trait ValidationRule {
    fn validate(&self, ir: &BpmnIR<Unvalidated>) -> Vec<ValidationError>;
}

/// Rule: Process must have a start event
pub struct StartEventRequiredRule;

impl ValidationRule for StartEventRequiredRule {
    fn validate(&self, ir: &BpmnIR<Unvalidated>) -> Vec<ValidationError> {
        let has_start = ir.elements().iter().any(|e| matches!(e, crate::core::elements::BpmnElement::StartEvent(_)));
        if !has_start {
            vec![ValidationError::error("Process must have at least one start event")
                .with_rule("start-event-required")]
        } else {
            Vec::new()
        }
    }
}

/// Rule: Process must have an end event
pub struct EndEventRequiredRule;

impl ValidationRule for EndEventRequiredRule {
    fn validate(&self, ir: &BpmnIR<Unvalidated>) -> Vec<ValidationError> {
        let has_end = ir.elements().iter().any(|e| matches!(e, crate::core::elements::BpmnElement::EndEvent(_)));
        if !has_end {
            vec![ValidationError::error("Process must have at least one end event")
                .with_rule("end-event-required")]
        } else {
            Vec::new()
        }
    }
}

