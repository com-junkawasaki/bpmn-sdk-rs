//! Subprocess builder.

use crate::core::elements::{SubProcess, BpmnElement};
use crate::core::types::ElementId;
use crate::dsl::process::ProcessBuilder;

/// Subprocess builder
pub struct SubProcessBuilder<PB> {
    id: ElementId,
    name: Option<String>,
    triggered_by_event: bool,
    process_ref: Option<String>,
    parent: PB,
}

impl<PB> SubProcessBuilder<PB> {
    pub fn new(id: impl Into<ElementId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            triggered_by_event: false,
            process_ref: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn triggered_by_event(mut self, triggered: bool) -> Self {
        self.triggered_by_event = triggered;
        self
    }

    pub fn process_ref(mut self, process_ref: impl Into<String>) -> Self {
        self.process_ref = Some(process_ref.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> SubProcessBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let subprocess = SubProcess {
            id: self.id,
            name: self.name,
            triggered_by_event: self.triggered_by_event,
            process_ref: self.process_ref,
        };
        self.parent.add_element(BpmnElement::SubProcess(subprocess))
    }
}

