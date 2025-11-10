//! Core Internal Representation (IR) for BPMN processes.
//!
//! This module defines the type-safe internal representation that is independent
//! of the bpmn-engine's internal structure. The IR uses phantom types to track
//! validation and compilation state at compile time.

pub mod process;
pub mod elements;
pub mod types;

pub use process::BpmnIR;
pub use types::{ProcessId, ElementId, ActivityId, ValidationState, Unvalidated, Validated, Compiled};

