#![no_std]
#![allow(dead_code)]

/// `( -- )` empty command: do nothing
fn nop() {}

/// `( -- )` stop system until hard reset
fn halt() {
    loop {}
}

// fn add<T>(i: T, j: T) -> T {
//     i + j
//    }
