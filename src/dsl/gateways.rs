//! Gateway builders (exclusive, parallel, inclusive).

use crate::core::elements::{ExclusiveGateway, ParallelGateway, InclusiveGateway, BpmnElement};
use crate::core::types::ElementId;
use crate::dsl::process::ProcessBuilder;

/// Exclusive gateway builder
pub struct ExclusiveGatewayBuilder<PB> {
    id: ElementId,
    name: Option<String>,
    parent: PB,
}

impl<PB> ExclusiveGatewayBuilder<PB> {
    pub fn new(id: impl Into<ElementId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> ExclusiveGatewayBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let gateway = ExclusiveGateway {
            id: self.id,
            name: self.name,
        };
        self.parent.add_element(BpmnElement::ExclusiveGateway(gateway))
    }
}

/// Parallel gateway builder
pub struct ParallelGatewayBuilder<PB> {
    id: ElementId,
    name: Option<String>,
    parent: PB,
}

impl<PB> ParallelGatewayBuilder<PB> {
    pub fn new(id: impl Into<ElementId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> ParallelGatewayBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let gateway = ParallelGateway {
            id: self.id,
            name: self.name,
        };
        self.parent.add_element(BpmnElement::ParallelGateway(gateway))
    }
}

/// Inclusive gateway builder
pub struct InclusiveGatewayBuilder<PB> {
    id: ElementId,
    name: Option<String>,
    parent: PB,
}

impl<PB> InclusiveGatewayBuilder<PB> {
    pub fn new(id: impl Into<ElementId>, parent: PB) -> Self {
        Self {
            id: id.into(),
            name: None,
            parent,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl<S: crate::dsl::process::ProcessState> InclusiveGatewayBuilder<ProcessBuilder<S>> {
    pub fn finish(self) -> ProcessBuilder<S> {
        let gateway = InclusiveGateway {
            id: self.id,
            name: self.name,
        };
        self.parent.add_element(BpmnElement::InclusiveGateway(gateway))
    }
}

