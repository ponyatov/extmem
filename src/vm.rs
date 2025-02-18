#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

/// D stack size
const Dsz: usize = 0x10;
/// R stack size
const Rsz: usize = 0x100;
/// M memory size, bytes
const Msz: usize = 0x1000;

/// data stack
static mut D: [i32; Dsz] = [0; Dsz];
/// D stack pointer
static mut Dp: u8 = 0;

/// return stack
static mut R: [u32; Rsz] = [0; Rsz];
/// R stack pointer
static mut Rp: u16 = 0;

/// main memory
static mut M: [u8; Msz] = [0; Msz];
/// compiler pointer
static mut Cp: u16 = 0;
/// instruction pointer
static mut Ip: u16 = 0;

/// VM command opcodes
enum Op {
    nop = 0x00,
    halt = 0xff,
    jmp = 0x01,
    qjmp = 0x02,
    call = 0x03,
    ret = 0x04,
}

/// `( -- )` empty command: do nothing
fn nop() {}

/// `( -- )` stop system until hard reset
fn halt() {
    loop {}
}

unsafe fn jmp(addr: u16) {
    assert!((addr as usize) < Msz);
    Ip = addr;
}

unsafe fn qjmp(addr: u16) {
    assert!(Dp > 0);
    Dp -= 1;
    if D[Dp as usize] == 0 {
        jmp(addr);
    }
}
