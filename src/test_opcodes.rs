#[cfg(test)]
use crate::*;

#[test]
fn adc_test() {
    let asm = Assembler::default();
    let asm = asm.adc(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x61, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x71, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x65, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x75, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x69, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x79, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x7D, 0xCD, 0xAB]);
}

#[test]
fn and_test() {
    let asm = Assembler::default();
    let asm = asm.and(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x21, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x31, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x25, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x35, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x29, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x39, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x3D, 0xCD, 0xAB]);
}

#[test]
fn asl_test() {
    let asm = Assembler::default();
    let asm = asm.asl(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x06, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x16, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl(A);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0A]);

    let asm = Assembler::default();
    let asm = asm.asl(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x1E, 0xCD, 0xAB]);
}

#[test]
fn bcc_test() {
    let asm = Assembler::default();
    let asm = asm.bcc(-128);
    let asm = asm.bcc(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x90, 0x80, 0x90, 0x7F]);
}

#[test]
fn bcs_test() {
    let asm = Assembler::default();
    let asm = asm.bcs(-128);
    let asm = asm.bcs(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB0, 0x80, 0xB0, 0x7F]);
}

#[test]
fn beq_test() {
    let asm = Assembler::default();
    let asm = asm.beq(-128);
    let asm = asm.beq(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF0, 0x80, 0xF0, 0x7F]);
}

#[test]
fn bit_test() {
    let asm = Assembler::default();
    let asm = asm.bit(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x24, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.bit(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2C, 0xCD, 0xAB]);
}

#[test]
fn bmi_test() {
    let asm = Assembler::default();
    let asm = asm.bmi(-128);
    let asm = asm.bmi(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x30, 0x80, 0x30, 0x7F]);
}

#[test]
fn brk_test() {
    let asm = Assembler::default();
    let asm = asm.brk();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x00]);
}

#[test]
fn bne_test() {
    let asm = Assembler::default();
    let asm = asm.bne(-128);
    let asm = asm.bne(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD0, 0x80, 0xD0, 0x7F]);
}

#[test]
fn bpl_test() {
    let asm = Assembler::default();
    let asm = asm.bpl(-128);
    let asm = asm.bpl(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x10, 0x80, 0x10, 0x7F]);
}

#[test]
fn bvc_test() {
    let asm = Assembler::default();
    let asm = asm.bvc(-128);
    let asm = asm.bvc(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x50, 0x80, 0x50, 0x7F]);
}

#[test]
fn bvs_test() {
    let asm = Assembler::default();
    let asm = asm.bvs(-128);
    let asm = asm.bvs(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x70, 0x80, 0x70, 0x7F]);
}

#[test]
fn clc_test() {
    let asm = Assembler::default();
    let asm = asm.clc();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x18]);
}

#[test]
fn cld_test() {
    let asm = Assembler::default();
    let asm = asm.cld();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD8]);
}

#[test]
fn cli_test() {
    let asm = Assembler::default();
    let asm = asm.cli();

    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x58]);
}

#[test]
fn clv_test() {
    let asm = Assembler::default();
    let asm = asm.clv();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB8]);
}

#[test]
fn cmp_test() {
    let asm = Assembler::default();
    let asm = asm.cmp(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCD, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xDD, 0xCD, 0xAB]);
}

#[test]
fn cpx_test() {
    let asm = Assembler::default();
    let asm = asm.cpx(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpx(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE4, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpx(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xEC, 0xCD, 0xAB]);
}

#[test]
fn cpy_test() {
    let asm = Assembler::default();
    let asm = asm.cpy(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpy(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC4, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpy(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCC, 0xCD, 0xAB]);
}

#[test]
fn dec_test() {
    let asm = Assembler::default();
    let asm = asm.dec(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xDE, 0xCD, 0xAB]);
}

#[test]
fn dex_test() {
    let asm = Assembler::default();
    let asm = asm.dex();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCA]);
}

#[test]
fn dey_test() {
    let asm = Assembler::default();
    let asm = asm.dey();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x88]);
}

#[test]
fn eor_test() {
    let asm = Assembler::default();
    let asm = asm.eor(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x41, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x51, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x45, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x55, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x49, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x59, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x5D, 0xCD, 0xAB]);
}

#[test]
fn inc_test() {
    let asm = Assembler::default();
    let asm = asm.inc(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xEE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xFE, 0xCD, 0xAB]);
}

#[test]
fn inx_test() {
    let asm = Assembler::default();
    let asm = asm.inx();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE8]);
}

#[test]
fn iny_test() {
    let asm = Assembler::default();
    let asm = asm.iny();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC8]);
}

#[test]
fn jmp_test() {
    let asm = Assembler::default();
    let asm = asm.jmp(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4C, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.jmp(global(global(0xABCD)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6C, 0xCD, 0xAB]);
}

#[test]
fn jsr_test() {
    let asm = Assembler::default();
    let asm = asm.jsr(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x20, 0xCD, 0xAB]);
}

#[test]
fn lda_test() {
    let asm = Assembler::default();
    let asm = asm.lda(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAD, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBD, 0xCD, 0xAB]);
}

#[test]
fn ldx_test() {
    let asm = Assembler::default();
    let asm = asm.ldx(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA2, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx(zp(Y + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBE, 0xCD, 0xAB]);
}

#[test]
fn ldy_test() {
    let asm = Assembler::default();
    let asm = asm.ldy(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldy(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAC, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldy(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBC, 0xCD, 0xAB]);
}

#[test]
fn lsr_test() {
    let asm = Assembler::default();
    let asm = asm.lsr(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x46, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x56, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr(A);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4A]);

    let asm = Assembler::default();
    let asm = asm.lsr(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x5E, 0xCD, 0xAB]);
}

#[test]
fn nop_test() {
    let asm = Assembler::default();
    let asm = asm.nop();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xEA]);
}

#[test]
fn ora_test() {
    let asm = Assembler::default();
    let asm = asm.ora(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x01, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x11, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x05, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x15, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x09, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x19, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x1D, 0xCD, 0xAB]);
}

#[test]
fn pha_test() {
    let asm = Assembler::default();
    let asm = asm.pha();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x48]);
}

#[test]
fn php_test() {
    let asm = Assembler::default();
    let asm = asm.php();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x08]);
}

#[test]
fn pla_test() {
    let asm = Assembler::default();
    let asm = asm.pla();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x68]);
}

#[test]
fn plp_test() {
    let asm = Assembler::default();
    let asm = asm.plp();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x28]);
}

#[test]
fn sei_test() {
    let asm = Assembler::default();
    let asm = asm.sei();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x78]);
}

#[test]
fn sed_test() {
    let asm = Assembler::default();
    let asm = asm.sed();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF8]);
}

#[test]
fn rol_test() {
    let asm = Assembler::default();
    let asm = asm.rol(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x26, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x36, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol(A);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2A]);

    let asm = Assembler::default();
    let asm = asm.rol(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x3E, 0xCD, 0xAB]);
}

#[test]
fn ror_test() {
    let asm = Assembler::default();
    let asm = asm.ror(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x66, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x76, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror(A);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6A]);

    let asm = Assembler::default();
    let asm = asm.ror(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x7E, 0xCD, 0xAB]);
}

#[test]
fn rti_test() {
    let asm = Assembler::default();
    let asm = asm.rti();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x40]);
}

#[test]
fn rts_test() {
    let asm = Assembler::default();
    let asm = asm.rts();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x60]);
}

#[test]
fn sbc_test() {
    let asm = Assembler::default();
    let asm = asm.sbc(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xED, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xFD, 0xCD, 0xAB]);
}

#[test]
fn sec_test() {
    let asm = Assembler::default();
    let asm = asm.sec();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x38]);
}

#[test]
fn sta_test() {
    let asm = Assembler::default();
    let asm = asm.sta(global(zp(X + 0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x81, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta(global(Y + zp(0xAB)));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x91, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x85, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x95, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta(global(Y + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x99, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta(global(X + 0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x9D, 0xCD, 0xAB]);
}

#[test]
fn stx_test() {
    let asm = Assembler::default();
    let asm = asm.stx(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x86, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.stx(zp(Y + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x96, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.stx(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8E, 0xCD, 0xAB]);
}

#[test]
fn sty_test() {
    let asm = Assembler::default();
    let asm = asm.sty(zp(0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x84, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sty(zp(X + 0xAB));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x94, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sty(global(0xABCD));
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8C, 0xCD, 0xAB]);
}

#[test]
fn tax_test() {
    let asm = Assembler::default();
    let asm = asm.tax();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAA]);
}

#[test]
fn tay_test() {
    let asm = Assembler::default();
    let asm = asm.tay();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA8]);
}

#[test]
fn tsx_test() {
    let asm = Assembler::default();
    let asm = asm.tsx();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBA]);
}

#[test]
fn txa_test() {
    let asm = Assembler::default();
    let asm = asm.txa();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8A]);
}

#[test]
fn txs_test() {
    let asm = Assembler::default();
    let asm = asm.txs();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x9A]);
}

#[test]
fn tya_test() {
    let asm = Assembler::default();
    let asm = asm.tya();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x98]);
}
