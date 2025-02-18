#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
// #![allow(unreachable_patterns)]

/// M memory size, bytes
const Msz: u16 = 0x1000;
/// D stack size
const Dsz: u8 = 0x10;
/// R stack size
const Rsz: u16 = 0x100;

/// main memory
static mut M: [u8; Msz as usize] = [0; Msz as usize];
/// compiler pointer
static mut Cp: u16 = 0;
/// instruction pointer
static mut Ip: u16 = 0;

/// data stack
static mut D: [i32; Dsz as usize] = [0; Dsz as usize];
/// D stack pointer
static mut Dp: u8 = 0;

/// return stack
static mut R: [u16; Rsz as usize] = [0; Rsz as usize];
/// R stack pointer
static mut Rp: u16 = 0;

/// address VM word size (D stack uses native int)
const cell: u16 = size_of::<u16>() as u16;

/// VM command opcode
#[repr(u8)]
pub enum Op {
    nop = 0x00,
    halt = 0xff,
    jmp = 0x01,
    qjmp = 0x02,
    call = 0x03,
    ret = 0x04,
    // add more VM commands as needed...
}

/// `( -- )` empty command: do nothing
fn nop() {}

/// `( -- )` stop system until hard reset
fn halt() {
    loop {}
}

fn abort() {
    halt();
}

unsafe fn jmp(addr: u16) {
    assert!(addr < Msz);
    Ip = addr;
}

unsafe fn qjmp(addr: u16) {
    assert!(Dp > 0);
    Dp -= 1;
    if D[Dp as usize] == 0 {
        jmp(addr);
    }
}

unsafe fn call(addr: u16) {
    assert!(Rp < Rsz);
    R[Rp as usize] = Ip + cell;
    Rp += 1;
    jmp(addr);
}

unsafe fn ret() {
    assert!(Rp > 0);
    Rp -= 1;
    jmp(R[Rp as usize] as u16);
}

use core::mem;

unsafe fn vm() {
    loop {
        // fetch opcode
        assert!(Ip < Msz);
        let op = M[Ip as usize];
        Ip += 1;

        // prefetch optional cmd1 parameter
        let l = M[(Ip + 0) as usize] as u16;
        let h = M[(Ip + 1) as usize] as u16;
        let param = (h << 8) | (l);

        // decode & run command
        match mem::transmute::<u8, Op>(op) {
            Op::nop => nop(),
            Op::halt => halt(),
            Op::jmp => jmp(param),
            Op::qjmp => qjmp(param),
            Op::call => call(param),
            Op::ret => ret(),
            // add more VM commands as needed...
            _ => abort(), // ??? how to force ???
        }
    }
}
