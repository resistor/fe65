pub struct Assembler {
    bytes: Vec<u8>,
}

pub trait ZeroPageIndexedXIndirectAddressing {
    fn zero_page_indexed_x_indirect(self, val: u8) -> Assembler;
}

pub trait ZeroPageAddressing {
    fn zero_page(self, val: u8) -> Assembler;
}

pub trait ImmediateAddressing {
    fn immediate(self, val: u8) -> Assembler;
}

pub trait AbsoluteAddressing {
    fn absolute(self, val: u16) -> Assembler;
}

pub trait AbsoluteIndirectAddressing {
    fn absolute_indirect(self, val: u16) -> Assembler;
}

pub trait ZeroPageIndirectIndexedYAddressing {
    fn zero_page_indirect_indexed_y(self, val: u8) -> Assembler;
}

pub trait ZeroPageIndexedXAddressing {
    fn zero_page_indexed_x(self, val: u8) -> Assembler;
}

pub trait ZeroPageIndexedYAddressing {
    fn zero_page_indexed_y(self, val: u8) -> Assembler;
}

pub trait AbsoluteIndexedYAddressing {
    fn absolute_indexed_y(self, val: u16) -> Assembler;
}

pub trait AbsoluteIndexedXAddressing {
    fn absolute_indexed_x(self, val: u16) -> Assembler;
}

pub trait AccumulatorAddressing {
    fn accumulator(self) -> Assembler;
}

pub trait RelativeAddressing {
    fn relative(self, value: i8) -> Assembler;
}

fn push_byte(mut bytes: Vec<u8>, b: u8) -> Vec<u8> {
    bytes.push(b);
    bytes
}

fn push_u16_le(mut bytes: Vec<u8>, val: u16) -> Vec<u8> {
    let le_bytes = val.to_le_bytes();
    bytes.extend(le_bytes);
    bytes
}

fn patch_last(mut bytes: Vec<u8>, val: u8) -> Vec<u8> {
    let last = bytes.last_mut().unwrap();
    *last |= val;
    bytes
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
        pub fn $name(self) -> impl RelativeAddressing {
            let bytes = push_byte(self.bytes, $val);
            BranchOpcode {
                assembler: Assembler { bytes: bytes },
            }
        }
    };
}

macro_rules! type_00_instruction {
    ($name:ident, $val:expr, $traits:ty) => {
        pub fn $name(self) -> $traits {
            let bytes = push_byte(self.bytes, $val);
            Type01Opcode {
                assembler: Assembler { bytes: bytes },
            }
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
            let bytes = push_byte(self.bytes, $val);
            Type10Opcode {
                assembler: Assembler { bytes: bytes },
            }
        }
    };
}

struct BranchOpcode {
    assembler: Assembler,
}

struct JsrOpcode {
    assembler: Assembler,
}

struct JmpOpcode {
    assembler: Assembler,
}

struct Type01Opcode {
    assembler: Assembler,
}

struct Type10Opcode {
    assembler: Assembler,
}

impl Assembler {
    single_byte_instruction!(brk, 0x00);
    type_01_instruction!(
        ora,
        0x01,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        asl,
        0x02,
        impl ZeroPageAddressing
            + AccumulatorAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(php, 0x08);
    branch_instruction!(bpl, 0x10);
    single_byte_instruction!(clc, 0x18);
    type_00_instruction!(
        bit,
        0x20,
        impl ImmediateAddressing
            + ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    type_01_instruction!(
        and,
        0x21,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        rol,
        0x22,
        impl ZeroPageAddressing
            + AccumulatorAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(plp, 0x28);
    branch_instruction!(bmi, 0x30);
    single_byte_instruction!(sec, 0x38);
    single_byte_instruction!(rti, 0x40);
    type_01_instruction!(
        eor,
        0x41,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        lsr,
        0x42,
        impl ZeroPageAddressing
            + AccumulatorAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(pha, 0x48);
    branch_instruction!(bvc, 0x50);
    single_byte_instruction!(cli, 0x58);
    single_byte_instruction!(rts, 0x60);
    type_01_instruction!(
        adc,
        0x61,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        ror,
        0x62,
        impl ZeroPageAddressing
            + AccumulatorAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(pla, 0x68);
    branch_instruction!(bvs, 0x70);
    single_byte_instruction!(sei, 0x78);
    type_00_instruction!(
        sty,
        0x80,
        impl ZeroPageAddressing + AbsoluteAddressing + ZeroPageIndexedXAddressing
    );
    type_01_instruction!(
        sta,
        0x81,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        stx,
        0x82,
        impl ZeroPageAddressing + AbsoluteAddressing + ZeroPageIndexedYAddressing
    );
    single_byte_instruction!(dey, 0x88);
    single_byte_instruction!(txa, 0x8A);
    branch_instruction!(bcc, 0x90);
    single_byte_instruction!(tya, 0x98);
    single_byte_instruction!(txs, 0x9A);
    type_00_instruction!(
        ldy,
        0xA0,
        impl ImmediateAddressing
            + ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    type_01_instruction!(
        lda,
        0xA1,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        ldx,
        0xA2,
        impl ImmediateAddressing
            + ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedYAddressing
            + AbsoluteIndexedYAddressing
    );
    single_byte_instruction!(tay, 0xA8);
    single_byte_instruction!(tax, 0xAA);
    branch_instruction!(bcs, 0xB0);
    single_byte_instruction!(clv, 0xB8);
    single_byte_instruction!(tsx, 0xBA);
    type_00_instruction!(
        cpy,
        0xC0,
        impl ImmediateAddressing + ZeroPageAddressing + AbsoluteAddressing
    );
    type_01_instruction!(
        cmp,
        0xC1,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        dec,
        0xC2,
        impl ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(iny, 0xC8);
    single_byte_instruction!(dex, 0xCA);
    branch_instruction!(bne, 0xD0);
    type_00_instruction!(
        cpx,
        0xE0,
        impl ImmediateAddressing + ZeroPageAddressing + AbsoluteAddressing
    );
    type_01_instruction!(
        sbc,
        0xE1,
        impl ZeroPageIndirectIndexedYAddressing
            + ZeroPageAddressing
            + ImmediateAddressing
            + AbsoluteAddressing
            + ZeroPageIndirectIndexedYAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedYAddressing
            + AbsoluteIndexedXAddressing
    );
    type_10_instruction!(
        inc,
        0xE2,
        impl ZeroPageAddressing
            + AbsoluteAddressing
            + ZeroPageIndexedXAddressing
            + AbsoluteIndexedXAddressing
    );
    single_byte_instruction!(inx, 0xE8);
    single_byte_instruction!(nop, 0xEA);
    single_byte_instruction!(cld, 0xD8);
    branch_instruction!(beq, 0xF0);
    single_byte_instruction!(sed, 0xF8);

    pub fn jsr(mut self) -> impl AbsoluteAddressing {
        self.bytes.push(0x20);
        JsrOpcode { assembler: self }
    }

    pub fn jmp(mut self) -> impl AbsoluteAddressing + AbsoluteIndirectAddressing {
        self.bytes.push(0x4C);
        JmpOpcode { assembler: self }
    }
}

impl AbsoluteAddressing for JsrOpcode {
    fn absolute(self, val: u16) -> Assembler {
        let bytes = push_u16_le(self.assembler.bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteAddressing for JmpOpcode {
    fn absolute(self, val: u16) -> Assembler {
        let bytes = push_u16_le(self.assembler.bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteIndirectAddressing for JmpOpcode {
    fn absolute_indirect(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0x20);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageIndexedXIndirectAddressing for Type01Opcode {
    fn zero_page_indexed_x_indirect(self, val: u8) -> Assembler {
        let bytes = push_byte(self.assembler.bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageAddressing for Type01Opcode {
    fn zero_page(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b00100);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ImmediateAddressing for Type01Opcode {
    fn immediate(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b01000);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteAddressing for Type01Opcode {
    fn absolute(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b01100);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageIndirectIndexedYAddressing for Type01Opcode {
    fn zero_page_indirect_indexed_y(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b10000);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageIndexedXAddressing for Type01Opcode {
    fn zero_page_indexed_x(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b10100);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteIndexedYAddressing for Type01Opcode {
    fn absolute_indexed_y(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b11000);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteIndexedXAddressing for Type01Opcode {
    fn absolute_indexed_x(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b11100);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ImmediateAddressing for Type10Opcode {
    fn immediate(self, val: u8) -> Assembler {
        let bytes = push_byte(self.assembler.bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageAddressing for Type10Opcode {
    fn zero_page(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b00100);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AccumulatorAddressing for Type10Opcode {
    fn accumulator(self) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b01000);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteAddressing for Type10Opcode {
    fn absolute(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b01100);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageIndexedXAddressing for Type10Opcode {
    fn zero_page_indexed_x(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b10100);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl ZeroPageIndexedYAddressing for Type10Opcode {
    fn zero_page_indexed_y(self, val: u8) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b10100);
        let bytes = push_byte(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteIndexedXAddressing for Type10Opcode {
    fn absolute_indexed_x(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b11100);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl AbsoluteIndexedYAddressing for Type10Opcode {
    fn absolute_indexed_y(self, val: u16) -> Assembler {
        let bytes = patch_last(self.assembler.bytes, 0b11100);
        let bytes = push_u16_le(bytes, val);
        Assembler { bytes: bytes }
    }
}

impl RelativeAddressing for BranchOpcode {
    fn relative(self, val: i8) -> Assembler {
        let bytes = push_byte(self.assembler.bytes, val as u8);
        Assembler { bytes: bytes }
    }
}

impl Assembler {
    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}
