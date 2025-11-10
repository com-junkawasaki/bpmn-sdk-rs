//! Integration tests for BPMN SDK

use bpmn_sdk::dsl::flow;
use bpmn_sdk::{validate, compile};

#[test]
fn test_simple_process_creation() {
    let process = flow("TestProcess", |f| {
        f.process("TestProcess", |p| {
            p.start_event("Start")
             .user_task("Task")
                .name("Test Task")
                .finish()
             .end_event("End")
             .build()
        })
    });

    assert_eq!(process.process_id().as_str(), "TestProcess");
}

#[test]
fn test_process_validation() {
    let process = flow("TestProcess", |f| {
        f.process("TestProcess", |p| {
            p.start_event("Start")
             .user_task("Task")
                .name("Test Task")
                .finish()
             .end_event("End")
             .build()
        })
    });

    let result = validate(&process);
    assert!(result.is_ok(), "Process should be valid");
    
    let validated = result.unwrap();
    let _compiled = compile(&validated);
}

#[test]
fn test_process_with_gateway() {
    let process = flow("TestProcess", |f| {
        f.process("TestProcess", |p| {
            p.start_event("Start")
             .exclusive_gateway("Gateway")
                .finish()
             .user_task("Task")
                .name("Test Task")
                .finish()
             .end_event("End")
             .build()
        })
    });

    let result = validate(&process);
    assert!(result.is_ok(), "Process with gateway should be valid");
}

