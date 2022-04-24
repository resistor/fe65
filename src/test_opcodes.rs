#[cfg(test)]
use crate::*;

#[test]
fn adc_test() {
    let asm = Assembler::default();
    let asm = asm.adc().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x61, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x71, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x65, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x75, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x69, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x79, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.adc().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x7D, 0xCD, 0xAB]);
}

#[test]
fn and_test() {
    let asm = Assembler::default();
    let asm = asm.and().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x21, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x31, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x25, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x35, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x29, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x39, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.and().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x3D, 0xCD, 0xAB]);
}

#[test]
fn asl_test() {
    let asm = Assembler::default();
    let asm = asm.asl().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x06, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x16, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl().accumulator();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0A]);

    let asm = Assembler::default();
    let asm = asm.asl().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.asl().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x1E, 0xCD, 0xAB]);
}

#[test]
fn bcc_test() {
    let asm = Assembler::default();
    let asm = asm.bcc().relative(-128);
    let asm = asm.bcc().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x90, 0x80, 0x90, 0x7F]);
}

#[test]
fn bcs_test() {
    let asm = Assembler::default();
    let asm = asm.bcs().relative(-128);
    let asm = asm.bcs().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB0, 0x80, 0xB0, 0x7F]);
}

#[test]
fn beq_test() {
    let asm = Assembler::default();
    let asm = asm.beq().relative(-128);
    let asm = asm.beq().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF0, 0x80, 0xF0, 0x7F]);
}

#[test]
fn bit_test() {
    let asm = Assembler::default();
    let asm = asm.bit().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x24, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.bit().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2C, 0xCD, 0xAB]);
}

#[test]
fn bmi_test() {
    let asm = Assembler::default();
    let asm = asm.bmi().relative(-128);
    let asm = asm.bmi().relative(127);
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
    let asm = asm.bne().relative(-128);
    let asm = asm.bne().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD0, 0x80, 0xD0, 0x7F]);
}

#[test]
fn bpl_test() {
    let asm = Assembler::default();
    let asm = asm.bpl().relative(-128);
    let asm = asm.bpl().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x10, 0x80, 0x10, 0x7F]);
}

#[test]
fn bvc_test() {
    let asm = Assembler::default();
    let asm = asm.bvc().relative(-128);
    let asm = asm.bvc().relative(127);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x50, 0x80, 0x50, 0x7F]);
}

#[test]
fn bvs_test() {
    let asm = Assembler::default();
    let asm = asm.bvs().relative(-128);
    let asm = asm.bvs().relative(127);
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
    let asm = asm.cmp().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCD, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cmp().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xDD, 0xCD, 0xAB]);
}

#[test]
fn cpx_test() {
    let asm = Assembler::default();
    let asm = asm.cpx().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpx().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE4, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpx().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xEC, 0xCD, 0xAB]);
}

#[test]
fn cpy_test() {
    let asm = Assembler::default();
    let asm = asm.cpy().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpy().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC4, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.cpy().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCC, 0xCD, 0xAB]);
}

#[test]
fn dec_test() {
    let asm = Assembler::default();
    let asm = asm.dec().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xC6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xD6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xCE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.dec().absolute_indexed_x(0xABCD);
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
    let asm = asm.eor().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x41, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x51, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x45, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x55, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x49, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x59, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.eor().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x5D, 0xCD, 0xAB]);
}

#[test]
fn inc_test() {
    let asm = Assembler::default();
    let asm = asm.inc().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xEE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.inc().absolute_indexed_x(0xABCD);
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
    let asm = asm.jmp().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4C, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.jmp().absolute_indirect(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6C, 0xCD, 0xAB]);
}

#[test]
fn jsr_test() {
    let asm = Assembler::default();
    let asm = asm.jsr().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x20, 0xCD, 0xAB]);
}

#[test]
fn lda_test() {
    let asm = Assembler::default();
    let asm = asm.lda().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAD, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lda().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBD, 0xCD, 0xAB]);
}

#[test]
fn ldx_test() {
    let asm = Assembler::default();
    let asm = asm.ldx().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA2, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx().zero_page_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xB6, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAE, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldx().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBE, 0xCD, 0xAB]);
}

#[test]
fn ldy_test() {
    let asm = Assembler::default();
    let asm = asm.ldy().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xA0, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldy().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xAC, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ldy().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xBC, 0xCD, 0xAB]);
}

#[test]
fn lsr_test() {
    let asm = Assembler::default();
    let asm = asm.lsr().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x46, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x56, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr().accumulator();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4A]);

    let asm = Assembler::default();
    let asm = asm.lsr().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x4E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.lsr().absolute_indexed_x(0xABCD);
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
    let asm = asm.ora().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x01, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x11, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x05, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x15, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x09, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x19, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x0D, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ora().absolute_indexed_x(0xABCD);
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
    let asm = asm.rol().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x26, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x36, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol().accumulator();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2A]);

    let asm = Assembler::default();
    let asm = asm.rol().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x2E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.rol().absolute_indexed_x(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x3E, 0xCD, 0xAB]);
}

#[test]
fn ror_test() {
    let asm = Assembler::default();
    let asm = asm.ror().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x66, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x76, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror().accumulator();
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6A]);

    let asm = Assembler::default();
    let asm = asm.ror().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x6E, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.ror().absolute_indexed_x(0xABCD);
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
    let asm = asm.sbc().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF1, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF5, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().immediate(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xE9, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xF9, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0xED, 0xCD, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sbc().absolute_indexed_x(0xABCD);
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
    let asm = asm.sta().zero_page_indexed_x_indirect(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x81, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta().zero_page_indirect_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x91, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x85, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x95, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sta().absolute_indexed_y(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x99, 0xCD, 0xAB]);
}

#[test]
fn stx_test() {
    let asm = Assembler::default();
    let asm = asm.stx().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x86, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.stx().zero_page_indexed_y(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x96, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.stx().absolute(0xABCD);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x8E, 0xCD, 0xAB]);
}

#[test]
fn sty_test() {
    let asm = Assembler::default();
    let asm = asm.sty().zero_page(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x84, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sty().zero_page_indexed_x(0xAB);
    let bytes = asm.take_bytes();
    assert_eq!(bytes, vec![0x94, 0xAB]);

    let asm = Assembler::default();
    let asm = asm.sty().absolute(0xABCD);
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
