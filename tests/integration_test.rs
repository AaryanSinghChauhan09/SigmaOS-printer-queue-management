// Integration test for SigmaOS-printer-queue-management
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use printer_queue_management as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
