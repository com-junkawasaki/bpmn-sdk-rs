//! BPMN element definitions for the IR.

use crate::core::types::{ElementId, ActivityId};
use serde::{Serialize, Deserialize};

/// BPMN element types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BpmnElement {
    /// Start event
    StartEvent(StartEvent),
    /// End event
    EndEvent(EndEvent),
    /// Intermediate event
    IntermediateEvent(IntermediateEvent),
    /// User task
    UserTask(UserTask),
    /// Service task
    ServiceTask(ServiceTask),
    /// Script task
    ScriptTask(ScriptTask),
    /// Manual task
    ManualTask(ManualTask),
    /// Exclusive gateway
    ExclusiveGateway(ExclusiveGateway),
    /// Parallel gateway
    ParallelGateway(ParallelGateway),
    /// Inclusive gateway
    InclusiveGateway(InclusiveGateway),
    /// Sequence flow
    SequenceFlow(SequenceFlow),
    /// Subprocess
    SubProcess(SubProcess),
}

/// Start event definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartEvent {
    pub id: ElementId,
    pub name: Option<String>,
    pub event_definition: Option<EventDefinition>,
}

/// End event definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndEvent {
    pub id: ElementId,
    pub name: Option<String>,
    pub event_definition: Option<EventDefinition>,
}

/// Intermediate event definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntermediateEvent {
    pub id: ElementId,
    pub name: Option<String>,
    pub event_definition: Option<EventDefinition>,
}

/// Event definition types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum EventDefinition {
    Message { name: Option<String> },
    Timer { expression: String },
    Signal { name: Option<String> },
    Error { error_code: Option<String> },
    Escalation { escalation_code: Option<String> },
}

/// User task definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserTask {
    pub id: ActivityId,
    pub name: Option<String>,
    pub assignee: Option<String>,
    pub candidate_users: Vec<String>,
    pub candidate_groups: Vec<String>,
    pub form_key: Option<String>,
    pub due_date: Option<String>,
}

/// Service task definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceTask {
    pub id: ActivityId,
    pub name: Option<String>,
    pub implementation: Option<String>,
    pub implementation_type: Option<String>,
}

/// Script task definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptTask {
    pub id: ActivityId,
    pub name: Option<String>,
    pub script: String,
    pub script_format: Option<String>,
}

/// Manual task definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManualTask {
    pub id: ActivityId,
    pub name: Option<String>,
}

/// Exclusive gateway definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExclusiveGateway {
    pub id: ElementId,
    pub name: Option<String>,
}

/// Parallel gateway definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelGateway {
    pub id: ElementId,
    pub name: Option<String>,
}

/// Inclusive gateway definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InclusiveGateway {
    pub id: ElementId,
    pub name: Option<String>,
}

/// Sequence flow definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceFlow {
    pub id: ElementId,
    pub name: Option<String>,
    pub source_ref: ElementId,
    pub target_ref: ElementId,
    pub condition_expression: Option<String>,
}

/// Subprocess definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubProcess {
    pub id: ElementId,
    pub name: Option<String>,
    pub triggered_by_event: bool,
    // Note: Subprocess can contain nested elements, but for simplicity
    // we'll reference another process definition
    pub process_ref: Option<String>,
}

