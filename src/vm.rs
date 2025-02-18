#![no_std]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

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

/// VM command opcodes
#[repr(u8)]
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

unsafe fn vm() {
    loop {
        assert!(Ip < Msz);
        let op = M[Ip as usize];
        Ip += 1;
        match op {
            Op::nop => nop(),
            Op::halt => halt(),
            _ => abort(),
        }
    }
}
