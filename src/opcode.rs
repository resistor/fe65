use crate::util::push_byte;
use crate::util::push_u16_le;
use crate::Assembler;

pub trait ZeroPageIndexedXIndirectAddressable {
    fn zero_page_indexed_x_indirect(self, val: u8) -> Assembler;
}

pub trait ZeroPageAddressable {
    fn zero_page(self, val: u8) -> Assembler;
}

pub trait ImmediateAddressable {
    fn immediate(self, val: u8) -> Assembler;
}

pub trait AbsoluteAddressable {
    fn absolute(self, val: u16) -> Assembler;
}

pub trait AbsoluteIndirectAddressable {
    fn absolute_indirect(self, val: u16) -> Assembler;
}

pub trait ZeroPageIndirectIndexedYAddressable {
    fn zero_page_indirect_indexed_y(self, val: u8) -> Assembler;
}

pub trait ZeroPageIndexedXAddressable {
    fn zero_page_indexed_x(self, val: u8) -> Assembler;
}

pub trait ZeroPageIndexedYAddressable {
    fn zero_page_indexed_y(self, val: u8) -> Assembler;
}

pub trait AbsoluteIndexedYAddressable {
    fn absolute_indexed_y(self, val: u16) -> Assembler;
}

pub trait AbsoluteIndexedXAddressable {
    fn absolute_indexed_x(self, val: u16) -> Assembler;
}

pub trait AccumulatorAddressable {
    fn accumulator(self) -> Assembler;
}

pub trait RelativeAddressable {
    fn relative(self, val: i8) -> Assembler;
}

pub trait AddressingType {
    const PATCH_IMMEDIATE: u8;
    const PATCH_ACCUMULATOR: u8;
    const PATCH_RELATIVE: u8;
    const PATCH_ABSOLUTE: u8;
    const PATCH_ABSOLUTE_INDIRECT: u8;
    const PATCH_ZERO_PAGE: u8;
    const PATCH_ZERO_PAGE_INDEXED_X: u8;
    const PATCH_ZERO_PAGE_INDEXED_Y: u8;
    const PATCH_ZERO_PAGE_INDEXED_X_INDIRECT: u8;
    const PATCH_ZERO_PAGE_INDIRECT_INDEXED_Y: u8;
    const PATCH_ABSOLUTE_INDEXED_X: u8;
    const PATCH_ABSOLUTE_INDEXED_Y: u8;
}

pub struct Jmp;
impl AddressingType for Jmp {
    const PATCH_IMMEDIATE: u8 = 0;
    const PATCH_ACCUMULATOR: u8 = 0;
    const PATCH_RELATIVE: u8 = 0;
    const PATCH_ABSOLUTE: u8 = 0;
    const PATCH_ABSOLUTE_INDIRECT: u8 = 0b00100000;
    const PATCH_ZERO_PAGE: u8 = 0;
    const PATCH_ZERO_PAGE_INDEXED_X: u8 = 0;
    const PATCH_ZERO_PAGE_INDEXED_Y: u8 = 0;
    const PATCH_ZERO_PAGE_INDEXED_X_INDIRECT: u8 = 0;
    const PATCH_ZERO_PAGE_INDIRECT_INDEXED_Y: u8 = 0;
    const PATCH_ABSOLUTE_INDEXED_X: u8 = 0;
    const PATCH_ABSOLUTE_INDEXED_Y: u8 = 0;
}

pub struct Type01;
impl AddressingType for Type01 {
    const PATCH_IMMEDIATE: u8 = 0b00001000;
    const PATCH_ACCUMULATOR: u8 = 0;
    const PATCH_RELATIVE: u8 = 0;
    const PATCH_ABSOLUTE: u8 = 0b00001100;
    const PATCH_ABSOLUTE_INDIRECT: u8 = 0;
    const PATCH_ZERO_PAGE: u8 = 0b00000100;
    const PATCH_ZERO_PAGE_INDEXED_X: u8 = 0b00010100;
    const PATCH_ZERO_PAGE_INDEXED_Y: u8 = 0;
    const PATCH_ZERO_PAGE_INDEXED_X_INDIRECT: u8 = 0;
    const PATCH_ZERO_PAGE_INDIRECT_INDEXED_Y: u8 = 0b00010000;
    const PATCH_ABSOLUTE_INDEXED_X: u8 = 0b00011100;
    const PATCH_ABSOLUTE_INDEXED_Y: u8 = 0b00011000;
}

pub struct Type10;
impl AddressingType for Type10 {
    const PATCH_IMMEDIATE: u8 = 0;
    const PATCH_ACCUMULATOR: u8 = 0b00001000;
    const PATCH_RELATIVE: u8 = 0;
    const PATCH_ABSOLUTE: u8 = 0b0001100;
    const PATCH_ABSOLUTE_INDIRECT: u8 = 0;
    const PATCH_ZERO_PAGE: u8 = 0b0000100;
    const PATCH_ZERO_PAGE_INDEXED_X: u8 = 0b0010100;
    const PATCH_ZERO_PAGE_INDEXED_Y: u8 = 0b0010100;
    const PATCH_ZERO_PAGE_INDEXED_X_INDIRECT: u8 = 0;
    const PATCH_ZERO_PAGE_INDIRECT_INDEXED_Y: u8 = 0;
    const PATCH_ABSOLUTE_INDEXED_X: u8 = 0b0011100;
    const PATCH_ABSOLUTE_INDEXED_Y: u8 = 0b0011100;
}

pub struct Opcode<T: AddressingType> {
    assembler: Assembler,
    opcode: u8,
    phantom: std::marker::PhantomData<T>,
}

impl<T: AddressingType> Opcode<T> {
    pub fn new(assembler: Assembler, opcode: u8) -> Opcode<T> {
        Opcode::<T> {
            assembler,
            opcode,
            phantom: std::marker::PhantomData,
        }
    }

    fn one_byte_instruction(self, patch: u8) -> Assembler {
        let bytes = push_byte(self.assembler.take_bytes(), self.opcode | patch);
        Assembler::from_bytes(bytes)
    }

    fn two_byte_instruction(self, patch: u8, val: u8) -> Assembler {
        let bytes = push_byte(self.assembler.take_bytes(), self.opcode | patch);
        let bytes = push_byte(bytes, val);
        Assembler::from_bytes(bytes)
    }

    fn three_byte_instruction(self, patch: u8, val: u16) -> Assembler {
        let bytes = push_byte(self.assembler.take_bytes(), self.opcode | patch);
        let bytes = push_u16_le(bytes, val);
        Assembler::from_bytes(bytes)
    }
}

impl<T: AddressingType> AbsoluteIndirectAddressable for Opcode<T> {
    fn absolute_indirect(self, val: u16) -> Assembler {
        self.three_byte_instruction(T::PATCH_ABSOLUTE_INDIRECT, val)
    }
}

impl<T: AddressingType> ZeroPageIndexedXIndirectAddressable for Opcode<T> {
    fn zero_page_indexed_x_indirect(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_ZERO_PAGE_INDEXED_X_INDIRECT, val)
    }
}

impl<T: AddressingType> ImmediateAddressable for Opcode<T> {
    fn immediate(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_IMMEDIATE, val)
    }
}

impl<T: AddressingType> AbsoluteAddressable for Opcode<T> {
    fn absolute(self, val: u16) -> Assembler {
        self.three_byte_instruction(T::PATCH_ABSOLUTE, val)
    }
}

impl<T: AddressingType> ZeroPageIndirectIndexedYAddressable for Opcode<T> {
    fn zero_page_indirect_indexed_y(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_ZERO_PAGE_INDIRECT_INDEXED_Y, val)
    }
}

impl<T: AddressingType> ZeroPageIndexedXAddressable for Opcode<T> {
    fn zero_page_indexed_x(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_ZERO_PAGE_INDEXED_X, val)
    }
}

impl<T: AddressingType> AbsoluteIndexedYAddressable for Opcode<T> {
    fn absolute_indexed_y(self, val: u16) -> Assembler {
        self.three_byte_instruction(T::PATCH_ABSOLUTE_INDEXED_Y, val)
    }
}

impl<T: AddressingType> AbsoluteIndexedXAddressable for Opcode<T> {
    fn absolute_indexed_x(self, val: u16) -> Assembler {
        self.three_byte_instruction(T::PATCH_ABSOLUTE_INDEXED_X, val)
    }
}

impl<T: AddressingType> ZeroPageAddressable for Opcode<T> {
    fn zero_page(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_ZERO_PAGE, val)
    }
}

impl<T: AddressingType> AccumulatorAddressable for Opcode<T> {
    fn accumulator(self) -> Assembler {
        self.one_byte_instruction(T::PATCH_ACCUMULATOR)
    }
}

impl<T: AddressingType> ZeroPageIndexedYAddressable for Opcode<T> {
    fn zero_page_indexed_y(self, val: u8) -> Assembler {
        self.two_byte_instruction(T::PATCH_ZERO_PAGE_INDEXED_Y, val)
    }
}

impl<T: AddressingType> RelativeAddressable for Opcode<T> {
    fn relative(self, val: i8) -> Assembler {
        self.two_byte_instruction(T::PATCH_RELATIVE, val as u8)
    }
}
