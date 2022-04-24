#[cfg(test)]
use crate::*;

#[test]
fn backward_label_absolute() {
    let asm = Assembler::default();
    let asm = asm.txa();
    let (asm, back) = asm.create_bound_label();
    let asm = asm.txa();
    let asm = asm.jmp().label(&back);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8A, 0x8A, 0x4C, 0x01, 0x00]);
}

#[test]
fn backward_label_relative() {
    let asm = Assembler::default();
    let asm = asm.txa();
    let (asm, back) = asm.create_bound_label();
    let asm = asm.txa();
    let asm = asm.bcc().label(&back);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8A, 0x8A, 0x90, 0xFF]);
}

#[test]
fn forward_label_absolute() {
    let asm = Assembler::default();
    let (asm, label) = asm.create_label();
    let asm = asm.jmp().label(&label);
    let asm = asm.txa();
    let asm = asm.bind_label(&label);
    let asm = asm.txa();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4C, 0x04, 0x00, 0x8A, 0x8A]);
}

#[test]
fn forward_label_relative() {
    let asm = Assembler::default();
    let (asm, label) = asm.create_label();
    let asm = asm.bcc().label(&label);
    let asm = asm.txa();
    let asm = asm.bind_label(&label);
    let asm = asm.txa();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x90, 0x03, 0x8A, 0x8A]);
}
