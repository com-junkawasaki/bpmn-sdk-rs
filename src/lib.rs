//! # BPMN SDK - Rust DSL
//!
//! Type-safe Rust DSL for declarative BPMN process modeling.
//!
//! ## Example
//!
//! ```rust
//! use bpmn_sdk::dsl::flow;
//!
//! let invoice_process = flow("InvoiceApproval", |f| {
//!     f.process("InvoiceApproval", |p| {
//!         p.start_event("StartEvent")
//!          .user_task("ReviewInvoice")
//!             .name("Review Invoice")
//!             .assignee("${managerId}")
//!             .finish()
//!          .exclusive_gateway("AmountCheck")
//!             .finish()
//!          .service_task("AutoApprove")
//!             .name("Auto Approve")
//!             .finish()
//!          .end_event("EndEvent")
//!          .build()
//!     })
//! });
//! ```

pub mod core;
pub mod dsl;
pub mod compiler;
pub mod validation;
pub mod human;
pub mod ops;
pub mod runtime;

// Re-export main types
pub use core::{BpmnIR, ProcessId, ElementId, ActivityId};
pub use dsl::flow;
pub use validation::{ValidationResult, ValidationError};

/// Convenience function to validate a BPMN IR
pub fn validate(ir: &BpmnIR<core::Unvalidated>) -> Result<BpmnIR<core::Validated>, ValidationResult> {
    validation::Validator::validate(ir)
}

/// Convenience function to compile a validated BPMN IR
pub fn compile(ir: &BpmnIR<core::Validated>) -> BpmnIR<core::Compiled> {
    compiler::Converter::compile(ir)
}

