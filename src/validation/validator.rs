//! Main validator implementation.

use crate::core::{BpmnIR, Unvalidated, Validated};
use crate::validation::reachability::ReachabilityAnalyzer;
use std::fmt;

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: ValidationError) {
        self.warnings.push(warning);
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl std::error::Error for ValidationResult {}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.errors.is_empty() {
            writeln!(f, "Validation errors:")?;
            for error in &self.errors {
                writeln!(f, "  - {}", error)?;
            }
        }
        if !self.warnings.is_empty() {
            writeln!(f, "Validation warnings:")?;
            for warning in &self.warnings {
                writeln!(f, "  - {}", warning)?;
            }
        }
        Ok(())
    }
}

/// Validation error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub severity: ValidationSeverity,
    pub message: String,
    pub element_id: Option<String>,
    pub rule: Option<String>,
}

impl ValidationError {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            message: message.into(),
            element_id: None,
            rule: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            message: message.into(),
            element_id: None,
            rule: None,
        }
    }

    pub fn with_element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }

    pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rule = Some(rule.into());
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.severity, self.message)?;
        if let Some(ref element_id) = self.element_id {
            write!(f, " (element: {})", element_id)?;
        }
        if let Some(ref rule) = self.rule {
            write!(f, " (rule: {})", rule)?;
        }
        Ok(())
    }
}

/// Validation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
}

impl fmt::Display for ValidationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationSeverity::Error => write!(f, "ERROR"),
            ValidationSeverity::Warning => write!(f, "WARNING"),
        }
    }
}

/// Main validator
pub struct Validator;

impl Validator {
    /// Validate a BPMN IR and return a validated version if successful
    pub fn validate(ir: &BpmnIR<Unvalidated>) -> Result<BpmnIR<Validated>, ValidationResult> {
        let mut result = ValidationResult::new();

        // Check for start event
        let has_start = ir.elements().iter().any(|e| matches!(e, crate::core::elements::BpmnElement::StartEvent(_)));
        if !has_start {
            result.add_error(ValidationError::error("Process must have at least one start event")
                .with_rule("start-event-required"));
        }

        // Check for end event
        let has_end = ir.elements().iter().any(|e| matches!(e, crate::core::elements::BpmnElement::EndEvent(_)));
        if !has_end {
            result.add_error(ValidationError::error("Process must have at least one end event")
                .with_rule("end-event-required"));
        }

        // Validate sequence flows
        Self::validate_sequence_flows(ir, &mut result);

        // Validate gateways
        Self::validate_gateways(ir, &mut result);

        // Check reachability
        let reachability = ReachabilityAnalyzer::analyze(ir);
        if !reachability.all_reachable {
            result.add_warning(ValidationError::warning("Some elements are not reachable from start event")
                .with_rule("reachability-check"));
        }

        if result.is_valid() {
            // Convert to validated state
            let validated = ir.clone().into_state();
            Ok(validated)
        } else {
            Err(result)
        }
    }

    fn validate_sequence_flows(ir: &BpmnIR<Unvalidated>, result: &mut ValidationResult) {
        use crate::core::elements::BpmnElement;
        
        let element_ids: std::collections::HashSet<_> = ir.elements()
            .iter()
            .filter_map(|e| match e {
                BpmnElement::StartEvent(se) => Some(se.id.as_str()),
                BpmnElement::EndEvent(ee) => Some(ee.id.as_str()),
                BpmnElement::UserTask(ut) => Some(ut.id.as_str()),
                BpmnElement::ServiceTask(st) => Some(st.id.as_str()),
                BpmnElement::ScriptTask(st) => Some(st.id.as_str()),
                BpmnElement::ManualTask(mt) => Some(mt.id.as_str()),
                BpmnElement::ExclusiveGateway(eg) => Some(eg.id.as_str()),
                BpmnElement::ParallelGateway(pg) => Some(pg.id.as_str()),
                BpmnElement::InclusiveGateway(ig) => Some(ig.id.as_str()),
                BpmnElement::SequenceFlow(_) => None, // Skip flows themselves
                _ => None,
            })
            .collect();

        for element in ir.elements() {
            if let BpmnElement::SequenceFlow(flow) = element {
                if !element_ids.contains(flow.source_ref.as_str()) {
                    result.add_error(ValidationError::error(format!("Sequence flow '{}' references unknown source element '{}'", flow.id.as_str(), flow.source_ref.as_str()))
                        .with_element_id(flow.id.as_str())
                        .with_rule("sequence-flow-source-valid"));
                }
                if !element_ids.contains(flow.target_ref.as_str()) {
                    result.add_error(ValidationError::error(format!("Sequence flow '{}' references unknown target element '{}'", flow.id.as_str(), flow.target_ref.as_str()))
                        .with_element_id(flow.id.as_str())
                        .with_rule("sequence-flow-target-valid"));
                }
            }
        }
    }

    fn validate_gateways(ir: &BpmnIR<Unvalidated>, result: &mut ValidationResult) {
        use crate::core::elements::BpmnElement;
        
        // Count incoming/outgoing flows for each gateway
        let mut incoming: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        let mut outgoing: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

        for element in ir.elements() {
            if let BpmnElement::SequenceFlow(flow) = element {
                *incoming.entry(flow.target_ref.as_str()).or_insert(0) += 1;
                *outgoing.entry(flow.source_ref.as_str()).or_insert(0) += 1;
            }
        }

        // Validate exclusive gateways
        for element in ir.elements() {
            if let BpmnElement::ExclusiveGateway(gateway) = element {
                let out_count = outgoing.get(gateway.id.as_str()).copied().unwrap_or(0);
                if out_count == 0 {
                    result.add_warning(ValidationError::warning(format!("Exclusive gateway '{}' has no outgoing flows", gateway.id.as_str()))
                        .with_element_id(gateway.id.as_str())
                        .with_rule("gateway-outgoing-flows"));
                }
            }
        }
    }
}

