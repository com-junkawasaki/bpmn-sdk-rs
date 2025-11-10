//! Example: Invoice approval process

use bpmn_sdk::dsl::flow;

fn main() {
    let invoice_process = flow("InvoiceApproval", |f| {
        f.process("InvoiceApproval", |p| {
            p.start_event("StartEvent")
             .user_task("ReviewInvoice")
                .name("Review Invoice")
                .assignee("${managerId}")
                .finish()
             .exclusive_gateway("AmountCheck")
                .finish()
             .service_task("AutoApprove")
                .name("Auto Approve")
                .finish()
             .end_event("EndEvent")
             .build()
        })
    });

    println!("Process created: {:?}", invoice_process.process_id());
    
    // Validate the process
    match bpmn_sdk::validate(&invoice_process) {
        Ok(validated) => {
            println!("Validation passed!");
            let compiled = bpmn_sdk::compile(&validated);
            println!("Compilation successful!");
            let _definition = compiled.into_definition();
            println!("Process definition ready for deployment");
        }
        Err(errors) => {
            eprintln!("Validation failed:");
            eprintln!("{}", errors);
        }
    }
}

