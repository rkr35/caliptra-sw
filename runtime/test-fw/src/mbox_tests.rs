/*++

Licensed under the Apache-2.0 license.

File Name:

    boot_tests.rs

Abstract:

    File contains test cases for booting runtime firmware

--*/

#![no_std]
#![no_main]

use caliptra_runtime::Drivers;
use caliptra_test_harness::test_suite;

fn test_mbox_cmd() {
    let mut drivers = unsafe { Drivers::new_from_registers() };
    caliptra_runtime::handle_mailbox_commands(&mut drivers);
}

test_suite! {
    test_mbox_cmd,
}
