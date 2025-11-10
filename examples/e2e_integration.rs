//! Example: End-to-end integration test

use bpmn_sdk::dsl::flow;

fn main() {
    // Create a simple approval process
    let approval_process = flow("SimpleApproval", |f| {
        f.process("SimpleApproval", |p| {
            p.start_event("Start")
             .user_task("Approve")
                .name("Approve Request")
                .finish()
             .end_event("End")
             .build()
        })
    });

    println!("Created process: {}", approval_process.process_id().as_str());
    
    // Validate
    match bpmn_sdk::validate(&approval_process) {
        Ok(validated) => {
            println!("✓ Validation passed");
            
            // Compile
            let compiled = bpmn_sdk::compile(&validated);
            println!("✓ Compilation successful");
            
            // Convert to definition (placeholder)
            let _definition = compiled.into_definition();
            println!("✓ Process definition ready");
        }
        Err(errors) => {
            eprintln!("✗ Validation failed:");
            eprintln!("{}", errors);
            std::process::exit(1);
        }
    }
}

