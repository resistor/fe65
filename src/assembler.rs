use crate::*;

#[derive(Default)]
pub struct Assembler {
    pub(crate) bytes: Vec<u8>,
    pub(crate) local_labels: Vec<Option<usize>>,
    pub(crate) relocations: Vec<Relocation>,
}

#[derive(Clone, Copy)]
pub struct Label(pub(crate) usize);

pub(crate) enum RelocationType {
    Absolute,
    RelativePlusOne,
}

pub(crate) struct Relocation {
    pub(crate) reloc: RelocationType,
    pub(crate) label: usize,
    pub(crate) vaddr: u16,
}

impl Assembler {
    pub fn from_bytes(bytes: Vec<u8>) -> Assembler {
        Assembler {
            bytes,
            ..Self::default()
        }
    }

    pub fn take_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn create_label(mut self) -> (Assembler, Label) {
        let idx = self.local_labels.len();
        self.local_labels.push(None);
        (self, Label(idx))
    }

    pub fn create_bound_label(mut self) -> (Assembler, Label) {
        let idx = self.local_labels.len();
        self.local_labels.push(Some(self.len()));
        (self, Label(idx))
    }

    pub fn bind_label(mut self, label: &Label) -> Assembler {
        let position = self.len() as usize;
        self.local_labels[label.0] = Some(position as usize);
        self.relocations
            .iter()
            .filter(|reloc| reloc.label == label.0)
            .for_each(|reloc| match &reloc.reloc {
                RelocationType::Absolute => {
                    let bytes = position.to_le_bytes();
                    self.bytes[reloc.vaddr as usize] = bytes[0];
                    self.bytes[(reloc.vaddr + 1) as usize] = bytes[1];
                }
                RelocationType::RelativePlusOne => {
                    let displacement = (position as isize) - (reloc.vaddr as isize) + 1;
                    let bytes = displacement.to_le_bytes();
                    self.bytes[reloc.vaddr as usize] = bytes[0];
                }
            });
        self.relocations.retain(|reloc| reloc.label == label.0);
        self
    }
}

macro_rules! single_byte_instruction {
    ($name:ident, $val:expr) => {
        pub fn $name(mut self) -> Assembler {
            self.bytes.push($val);
            self
        }
    };
}

macro_rules! multi_byte_instruction {
    ($name:ident, $opc:expr, $trait:ty) => {
        pub fn $name(mut self, address: $trait) -> Assembler {
            const OPC: u8 = $opc;
            self.bytes.push(OPC | address.addressing_mode());
            address.push_address_bytes(self)
        }
    };
}

impl Assembler {
    single_byte_instruction!(brk, 0x00);
    multi_byte_instruction!(ora, 0x01, impl Type01OperandEncoding);
    multi_byte_instruction!(asl, 0x02, impl Type10OperandEncodingShift);
    single_byte_instruction!(php, 0x08);
    multi_byte_instruction!(bpl, 0x10, impl OperandEncodingBRA);
    single_byte_instruction!(clc, 0x18);
    multi_byte_instruction!(jsr, 0x20, impl OperandEncodingJSR);
    multi_byte_instruction!(bit, 0x20, impl Type00OperandEncodingBIT);
    multi_byte_instruction!(and, 0x21, impl Type01OperandEncoding);
    multi_byte_instruction!(rol, 0x22, impl Type10OperandEncodingShift);
    single_byte_instruction!(plp, 0x28);
    multi_byte_instruction!(bmi, 0x30, impl OperandEncodingBRA);
    single_byte_instruction!(sec, 0x38);
    single_byte_instruction!(rti, 0x40);
    multi_byte_instruction!(eor, 0x41, impl Type01OperandEncoding);
    multi_byte_instruction!(lsr, 0x42, impl Type10OperandEncodingShift);
    single_byte_instruction!(pha, 0x48);
    multi_byte_instruction!(jmp, 0x4C, impl Type00OperandEncodingJMP);
    multi_byte_instruction!(bvc, 0x50, impl OperandEncodingBRA);
    single_byte_instruction!(cli, 0x58);
    single_byte_instruction!(rts, 0x60);
    multi_byte_instruction!(adc, 0x61, impl Type01OperandEncoding);
    multi_byte_instruction!(ror, 0x62, impl Type10OperandEncodingShift);
    single_byte_instruction!(pla, 0x68);
    multi_byte_instruction!(bvs, 0x70, impl OperandEncodingBRA);
    single_byte_instruction!(sei, 0x78);
    multi_byte_instruction!(sty, 0x80, impl Type00OperandEncodingSTY);
    multi_byte_instruction!(sta, 0x81, impl Type01OperandEncodingExceptImmediate);
    multi_byte_instruction!(stx, 0x82, impl Type10OperandEncodingSTX);
    single_byte_instruction!(dey, 0x88);
    single_byte_instruction!(txa, 0x8A);
    multi_byte_instruction!(bcc, 0x90, impl OperandEncodingBRA);
    single_byte_instruction!(tya, 0x98);
    single_byte_instruction!(txs, 0x9A);
    multi_byte_instruction!(ldy, 0xA0, impl Type00OperandEncodingLDY);
    multi_byte_instruction!(lda, 0xA1, impl Type01OperandEncoding);
    multi_byte_instruction!(ldx, 0xA2, impl Type10OperandEncodingLDX);
    single_byte_instruction!(tay, 0xA8);
    single_byte_instruction!(tax, 0xAA);
    multi_byte_instruction!(bcs, 0xB0, impl OperandEncodingBRA);
    single_byte_instruction!(clv, 0xB8);
    single_byte_instruction!(tsx, 0xBA);
    multi_byte_instruction!(cpy, 0xC0, impl Type00OperandEncodingCPY);
    multi_byte_instruction!(cmp, 0xC1, impl Type01OperandEncoding);
    multi_byte_instruction!(dec, 0xC2, impl Type10OperandEncodingINC);
    single_byte_instruction!(iny, 0xC8);
    single_byte_instruction!(dex, 0xCA);
    multi_byte_instruction!(bne, 0xD0, impl OperandEncodingBRA);
    multi_byte_instruction!(cpx, 0xE0, impl Type00OperandEncodingCPY);
    multi_byte_instruction!(sbc, 0xE1, impl Type01OperandEncoding);
    multi_byte_instruction!(inc, 0xE2, impl Type10OperandEncodingINC);
    single_byte_instruction!(inx, 0xE8);
    single_byte_instruction!(nop, 0xEA);
    single_byte_instruction!(cld, 0xD8);
    multi_byte_instruction!(beq, 0xF0, impl OperandEncodingBRA);
    single_byte_instruction!(sed, 0xF8);
}
