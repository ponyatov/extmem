#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

/// D stack size
const Dsz: usize = 0x10;
/// R stack size
const Rsz: usize = 0x100;
/// M memory size, bytes
const Msz: usize = 0x1000;

/// data stack
static D: [i32; Dsz] = [0; Dsz];
/// D stack pointer
static Dp: u8 = 0;

/// return stack
static R: [u32; Rsz] = [0; Rsz];
/// R stack pointer
static Rp: u16 = 0;

/// main memory
static M: [u8; Msz] = [0; Msz];
/// compiler pointer
static Cp: u16 = 0;
/// instruction pointer
static Ip: u16 = 0;

// fn add<T>(i: T, j: T) -> T {
//     i + j
//    }

/// `( -- )` empty command: do nothing
fn nop() {}

/// `( -- )` stop system until hard reset
fn halt() {
    loop {}
}
