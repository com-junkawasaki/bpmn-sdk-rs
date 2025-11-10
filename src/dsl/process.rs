//! Process builder with type state pattern.

use crate::core::{BpmnIR, ProcessId, Unvalidated};
use crate::core::elements::BpmnElement;
use crate::core::types::ElementId;
use crate::dsl::events::{StartEventBuilder, EndEventBuilder};
use crate::dsl::tasks::{UserTaskBuilder, ServiceTaskBuilder, ScriptTaskBuilder, ManualTaskBuilder};
use crate::dsl::gateways::{ExclusiveGatewayBuilder, ParallelGatewayBuilder, InclusiveGatewayBuilder};
use crate::dsl::flows::SequenceFlowBuilder;

/// Process builder state markers
pub trait ProcessState {}

/// Uninitialized state - must call process_id()
pub struct Uninitialized;
impl ProcessState for Uninitialized {}

/// Initialized state - can add elements
pub struct Initialized;
impl ProcessState for Initialized {}

/// Has start event state
pub struct HasStart;
impl ProcessState for HasStart {}

/// Has end event state
pub struct HasEnd;
impl ProcessState for HasEnd {}

/// Complete state - ready to build
pub struct Complete;
impl ProcessState for Complete {}

/// Process builder with type state pattern
pub struct ProcessBuilder<S: ProcessState = Uninitialized> {
    process_id: ProcessId,
    name: Option<String>,
    elements: Vec<BpmnElement>,
    variables: Vec<crate::core::process::ProcessVariable>,
    _state: std::marker::PhantomData<S>,
}

impl ProcessBuilder<Uninitialized> {
    pub fn new(process_id: impl Into<ProcessId>) -> Self {
        Self {
            process_id: process_id.into(),
            name: None,
            elements: Vec::new(),
            variables: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }
}

impl<S: ProcessState> ProcessBuilder<S> {
    /// Set the process name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add an element to the process (internal helper for builders)
    pub(crate) fn add_element(mut self, element: BpmnElement) -> Self {
        self.elements.push(element);
        self
    }

    /// Add a start event (transitions to HasStart state)
    pub fn start_event(self, id: impl Into<ElementId>) -> ProcessBuilder<HasStart> {
        let mut builder = self.transition();
        let start_event = StartEventBuilder::new(id).build();
        builder.elements.push(BpmnElement::StartEvent(start_event));
        builder
    }


    /// Add a user task
    pub fn user_task(self, id: impl Into<crate::core::types::ActivityId>) -> UserTaskBuilder<Self> {
        UserTaskBuilder::new(id, self)
    }

    /// Add a service task
    pub fn service_task(self, id: impl Into<crate::core::types::ActivityId>) -> ServiceTaskBuilder<Self> {
        ServiceTaskBuilder::new(id, self)
    }

    /// Add a script task
    pub fn script_task(self, id: impl Into<crate::core::types::ActivityId>) -> ScriptTaskBuilder<Self> {
        ScriptTaskBuilder::new(id, self)
    }

    /// Add a manual task
    pub fn manual_task(self, id: impl Into<crate::core::types::ActivityId>) -> ManualTaskBuilder<Self> {
        ManualTaskBuilder::new(id, self)
    }

    /// Add an exclusive gateway
    pub fn exclusive_gateway(self, id: impl Into<ElementId>) -> ExclusiveGatewayBuilder<Self> {
        ExclusiveGatewayBuilder::new(id, self)
    }

    /// Add a parallel gateway
    pub fn parallel_gateway(self, id: impl Into<ElementId>) -> ParallelGatewayBuilder<Self> {
        ParallelGatewayBuilder::new(id, self)
    }

    /// Add an inclusive gateway
    pub fn inclusive_gateway(self, id: impl Into<ElementId>) -> InclusiveGatewayBuilder<Self> {
        InclusiveGatewayBuilder::new(id, self)
    }

    /// Add a sequence flow
    pub fn sequence_flow(self, id: impl Into<ElementId>) -> SequenceFlowBuilder<Self> {
        SequenceFlowBuilder::new(id, self)
    }

    /// Transition to a new state (internal helper)
    fn transition<T: ProcessState>(self) -> ProcessBuilder<T> {
        ProcessBuilder {
            process_id: self.process_id,
            name: self.name,
            elements: self.elements,
            variables: self.variables,
            _state: std::marker::PhantomData,
        }
    }
}

impl ProcessBuilder<Complete> {
    /// Build the final BPMN IR
    pub fn build(self) -> BpmnIR<Unvalidated> {
        let mut ir = BpmnIR::new(self.process_id);
        if let Some(name) = self.name {
            ir = ir.with_name(name);
        }
        for element in self.elements {
            ir = ir.add_element(element);
        }
        for variable in self.variables {
            ir = ir.add_variable(variable);
        }
        ir
    }
}

// Allow building from HasEnd state (process must have both start and end)
impl ProcessBuilder<HasEnd> {
    pub fn build(self) -> BpmnIR<Unvalidated> {
        // Convert to Complete state and build
        ProcessBuilder::<Complete>::build(self.transition())
    }
}

// Allow end_event from HasStart state (after adding tasks/gateways)
impl ProcessBuilder<HasStart> {
    /// Add an end event (transitions to HasEnd state)
    pub fn end_event(self, id: impl Into<ElementId>) -> ProcessBuilder<HasEnd> {
        let mut builder = self.transition();
        let end_event = EndEventBuilder::new(id).build();
        builder.elements.push(BpmnElement::EndEvent(end_event));
        builder
    }
}

// Also allow end_event from Initialized state (before start event)
impl ProcessBuilder<Initialized> {
    /// Add an end event (transitions to HasEnd state)
    /// 
    /// Note: Process should have a start event before calling this
    pub fn end_event(self, id: impl Into<ElementId>) -> ProcessBuilder<HasEnd> {
        let mut builder = self.transition();
        let end_event = EndEventBuilder::new(id).build();
        builder.elements.push(BpmnElement::EndEvent(end_event));
        builder
    }
}

