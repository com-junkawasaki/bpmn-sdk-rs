# BPMN SDK - Rust DSL

Type-safe Rust DSL for declarative BPMN process modeling. This SDK provides a fluent, compile-time validated API for building BPMN 2.0 process definitions that compile to `bpmn-engine`'s `ProcessDefinition` format.

## Features

- ✅ **Type-Safe DSL**: Builder pattern with type state ensures correct process construction at compile time
- ✅ **Static Validation**: Validate process definitions before compilation
- ✅ **Human Task Management**: Built-in support for user tasks, SLAs, and escalation
- ✅ **Monitoring & Observability**: Metrics, tracing, and alerting support
- ✅ **Runtime Integration**: Seamless integration with `bpmn-engine`

## Quick Start

```rust
use bpmn_sdk::dsl::flow;

let invoice_process = flow("InvoiceApproval", |f| {
    f.process("InvoiceApproval", |p| {
        p.start_event("StartEvent")
         .user_task("ReviewInvoice")
            .name("Review Invoice")
            .assignee("${managerId}")
         .exclusive_gateway("AmountCheck")
         .service_task("AutoApprove")
            .condition("${amount <= 1000}")
         .end_event("EndEvent")
         .build()
    })
});

let validated = invoice_process.validate()?;
let compiled = validated.compile();
let definition = compiled.into_definition(); // ProcessDefinition for bpmn-engine
```

## Architecture

- **Core IR**: Internal representation independent of engine internals
- **DSL Layer**: Type-safe builders for all BPMN elements
- **Compiler**: Converts IR to `ProcessDefinition`
- **Validation**: Static validation with detailed error messages
- **Human Tasks**: Task management, SLA tracking, escalation
- **Ops**: Metrics, tracing, and alerting

## License

MIT OR Apache-2.0

