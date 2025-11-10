//! Flow DSL entry point.

use crate::core::{BpmnIR, ProcessId, Unvalidated};
use crate::dsl::process::ProcessBuilder;

/// Flow builder - entry point for DSL
pub struct FlowBuilder {
    #[allow(dead_code)]
    name: String,
}

impl FlowBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// Start building a process
    pub fn process<F>(self, process_id: impl Into<ProcessId>, builder: F) -> BpmnIR<Unvalidated>
    where
        F: FnOnce(ProcessBuilder<crate::dsl::process::Uninitialized>) -> crate::core::BpmnIR<crate::core::Unvalidated>,
    {
        let process_builder = ProcessBuilder::new(process_id);
        builder(process_builder)
    }
}

/// Entry point function for creating a flow
pub fn flow<F>(name: impl Into<String>, builder: F) -> BpmnIR<Unvalidated>
where
    F: FnOnce(FlowBuilder) -> BpmnIR<Unvalidated>,
{
    let flow_builder = FlowBuilder::new(name);
    builder(flow_builder)
}

