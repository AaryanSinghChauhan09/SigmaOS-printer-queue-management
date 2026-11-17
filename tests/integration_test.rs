//! Integration tests for SigmaOS-printer-queue-management.

use sigmaos_printer_queue_management_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
