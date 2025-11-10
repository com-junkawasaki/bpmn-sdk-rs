//! Example: Order processing workflow

use bpmn_sdk::dsl::flow;

fn main() {
    let order_process = flow("OrderProcessing", |f| {
        f.process("OrderProcessing", |p| {
            p.name("Order Processing Workflow")
             .start_event("OrderReceived")
             .user_task("ValidateOrder")
                .name("Validate Order")
                .candidate_group("order-validators")
                .finish()
             .parallel_gateway("SplitValidation")
                .finish()
             .service_task("CheckInventory")
                .name("Check Inventory")
                .implementation("inventory-service")
                .finish()
             .service_task("CheckPayment")
                .name("Check Payment")
                .implementation("payment-service")
                .finish()
             .parallel_gateway("JoinValidation")
                .finish()
             .exclusive_gateway("OrderDecision")
                .finish()
             .user_task("FulfillOrder")
                .name("Fulfill Order")
                .candidate_group("fulfillment-team")
                .finish()
             .end_event("OrderCompleted")
             .build()
        })
    });

    println!("Order process created: {:?}", order_process.process_id());
    
    match bpmn_sdk::validate(&order_process) {
        Ok(validated) => {
            println!("Validation passed!");
            let compiled = bpmn_sdk::compile(&validated);
            println!("Compilation successful!");
        }
        Err(errors) => {
            eprintln!("Validation failed:");
            eprintln!("{}", errors);
        }
    }
}

