//! Sequence flow builder.

use crate::core::elements::{SequenceFlow, BpmnElement};
use crate::core::types::ElementId;
use crate::dsl::process::ProcessBuilder;

/// Sequence flow builder
pub struct SequenceFlowBuilder<PB> {
    id: ElementId,
    name: Option<String>,
    source_ref: Option<ElementId>,
    target_ref: Option<ElementId>,
    condition_expression: Option<String>,
    parent: PB,
}

impl<PB> SequenceFlowBuilder<PB> {
    pub fn new(id: impl Into<ElementId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            source_ref: None,
            target_ref: None,
            condition_expression: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn from(mut self, source_ref: impl Into<ElementId>) -> Self {
        self.source_ref = Some(source_ref.into());
        self
    }

    pub fn to(mut self, target_ref: impl Into<ElementId>) -> Self {
        self.target_ref = Some(target_ref.into());
        self
    }

    pub fn condition(mut self, expression: impl Into<String>) -> Self {
        self.condition_expression = Some(expression.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> SequenceFlowBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let flow = SequenceFlow {
            id: self.id,
            name: self.name,
            source_ref: self.source_ref.unwrap_or_else(|| ElementId::new("")),
            target_ref: self.target_ref.unwrap_or_else(|| ElementId::new("")),
            condition_expression: self.condition_expression,
        };
        self.parent.add_element(BpmnElement::SequenceFlow(flow))
    }
}

