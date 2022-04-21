use crate::opcode::{Jmp, Opcode, Type01, Type10};
use crate::util::push_byte;
use crate::{
    AbsoluteAddressable, AbsoluteIndexedXAddressable, AbsoluteIndexedYAddressable,
    AbsoluteIndirectAddressable, AccumulatorAddressable, ImmediateAddressable, RelativeAddressable,
    ZeroPageAddressable, ZeroPageIndexedXAddressable, ZeroPageIndexedYAddressable,
    ZeroPageIndirectIndexedYAddressable,
};

#[derive(Default)]
pub struct Assembler {
    bytes: Vec<u8>,
}

impl Assembler {
    pub fn from_bytes(bytes: Vec<u8>) -> Assembler {
        Assembler { bytes }
    }

    pub fn take_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

macro_rules! single_byte_instruction {
    ($name:ident, $val:expr) => {
        pub fn $name(self) -> Assembler {
            Assembler {
                bytes: push_byte(self.bytes, $val),
            }
        }
    };
}

macro_rules! branch_instruction {
    ($name:ident, $val:expr) => {
        pub fn $name(self) -> impl RelativeAddressable {
            Opcode::<Jmp>::new(self, $val)
        }
    };
}

macro_rules! type_00_instruction {
    ($name:ident, $val:expr, $traits:ty) => {
        pub fn $name(self) -> $traits {
            Opcode::<Type01>::new(self, $val)
        }
    };
}

macro_rules! type_01_instruction {
    ($name:ident, $val:expr, $traits:ty) => {
        type_00_instruction!($name, $val, $traits);
    };
}

macro_rules! type_10_instruction {
    ($name:ident, $val:expr, $traits:ty) => {
        pub fn $name(self) -> $traits {
            Opcode::<Type10>::new(self, $val)
        }
    };
}

impl Assembler {
    single_byte_instruction!(brk, 0x00);
    type_01_instruction!(
        ora,
        0x01,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        asl,
        0x02,
        impl ZeroPageAddressable
            + AccumulatorAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(php, 0x08);
    branch_instruction!(bpl, 0x10);
    single_byte_instruction!(clc, 0x18);
    type_00_instruction!(
        bit,
        0x20,
        impl ImmediateAddressable
            + ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    type_01_instruction!(
        and,
        0x21,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        rol,
        0x22,
        impl ZeroPageAddressable
            + AccumulatorAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(plp, 0x28);
    branch_instruction!(bmi, 0x30);
    single_byte_instruction!(sec, 0x38);
    single_byte_instruction!(rti, 0x40);
    type_01_instruction!(
        eor,
        0x41,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        lsr,
        0x42,
        impl ZeroPageAddressable
            + AccumulatorAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(pha, 0x48);
    branch_instruction!(bvc, 0x50);
    single_byte_instruction!(cli, 0x58);
    single_byte_instruction!(rts, 0x60);
    type_01_instruction!(
        adc,
        0x61,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        ror,
        0x62,
        impl ZeroPageAddressable
            + AccumulatorAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(pla, 0x68);
    branch_instruction!(bvs, 0x70);
    single_byte_instruction!(sei, 0x78);
    type_00_instruction!(
        sty,
        0x80,
        impl ZeroPageAddressable + AbsoluteAddressable + ZeroPageIndexedXAddressable
    );
    type_01_instruction!(
        sta,
        0x81,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        stx,
        0x82,
        impl ZeroPageAddressable + AbsoluteAddressable + ZeroPageIndexedYAddressable
    );
    single_byte_instruction!(dey, 0x88);
    single_byte_instruction!(txa, 0x8A);
    branch_instruction!(bcc, 0x90);
    single_byte_instruction!(tya, 0x98);
    single_byte_instruction!(txs, 0x9A);
    type_00_instruction!(
        ldy,
        0xA0,
        impl ImmediateAddressable
            + ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    type_01_instruction!(
        lda,
        0xA1,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        ldx,
        0xA2,
        impl ImmediateAddressable
            + ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedYAddressable
            + AbsoluteIndexedYAddressable
    );
    single_byte_instruction!(tay, 0xA8);
    single_byte_instruction!(tax, 0xAA);
    branch_instruction!(bcs, 0xB0);
    single_byte_instruction!(clv, 0xB8);
    single_byte_instruction!(tsx, 0xBA);
    type_00_instruction!(
        cpy,
        0xC0,
        impl ImmediateAddressable + ZeroPageAddressable + AbsoluteAddressable
    );
    type_01_instruction!(
        cmp,
        0xC1,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        dec,
        0xC2,
        impl ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(iny, 0xC8);
    single_byte_instruction!(dex, 0xCA);
    branch_instruction!(bne, 0xD0);
    type_00_instruction!(
        cpx,
        0xE0,
        impl ImmediateAddressable + ZeroPageAddressable + AbsoluteAddressable
    );
    type_01_instruction!(
        sbc,
        0xE1,
        impl ZeroPageIndirectIndexedYAddressable
            + ZeroPageAddressable
            + ImmediateAddressable
            + AbsoluteAddressable
            + ZeroPageIndirectIndexedYAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedYAddressable
            + AbsoluteIndexedXAddressable
    );
    type_10_instruction!(
        inc,
        0xE2,
        impl ZeroPageAddressable
            + AbsoluteAddressable
            + ZeroPageIndexedXAddressable
            + AbsoluteIndexedXAddressable
    );
    single_byte_instruction!(inx, 0xE8);
    single_byte_instruction!(nop, 0xEA);
    single_byte_instruction!(cld, 0xD8);
    branch_instruction!(beq, 0xF0);
    single_byte_instruction!(sed, 0xF8);

    pub fn jsr(self) -> impl AbsoluteAddressable {
        Opcode::<Jmp>::new(self, 0x20)
    }

    pub fn jmp(self) -> impl AbsoluteAddressable + AbsoluteIndirectAddressable {
        Opcode::<Jmp>::new(self, 0x4C)
    }
}
