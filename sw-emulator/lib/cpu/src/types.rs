/*++

Licensed under the Apache-2.0 license.

File Name:

    types.rs

Abstract:

    Common types used in the project.

--*/

#![allow(clippy::unusual_byte_groupings)]

use crate::xreg_file::XReg;
use bitfield::{bitfield, BitRange, BitRangeMut};
use caliptra_emu_types::emu_enum;

emu_enum! {
    /// RISCV 32-bit instruction opcodes
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32Opcode;
    u32;
    {
        /// Load Instruction Opcode
        Load = 0b000_0011,

        /// Op Immediate Opcode
        OpImm = 0b001_0011,

        /// AUIPC Opcode
        Auipc = 0b001_0111,

        /// Store Instruction Opcode
        Store = 0b010_0011,

        /// Operation Instruction Opcode
        Op = 0b011_0011,

        /// LUI Opcode
        Lui = 0b011_0111,

        /// Branch Opcode
        Branch = 0b110_0011,

        /// Jump and Link Register Opcode
        Jalr = 0b110_0111,

        /// Jump and Link Opcode
        Jal = 0b110_1111,

        /// System Opcode
        System = 0b111_0011,
    };
    Invalid
}

emu_enum! {
    /// RISCV Load Opcode Functions
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32LoadFunct3;
    u32;
    {
        /// Load Byte function
        Lb = 0b000,

        /// Load Half Word function
        Lh = 0b001,

        /// Load Word function
        Lw = 0b010,

        /// Load Byte Unsigned function
        Lbu = 0b100,

        /// Load Half Word Unsigned function
        Lhu = 0b101,
    };
    Invalid
}

emu_enum! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32OpImmFunct3;
    u32;
    {
        /// Add immediate
        Addi = 0b000,

        /// Shift left immediate
        Sli = 0b001,

        /// Set less than immediate
        Slti = 0b010,

        /// Set less than immediate unsigned
        Sltiu = 0b011,

        /// Xor immediate
        Xori = 0b100,

        /// Shift right immediate
        Sri = 0b101,

        /// Or immediate
        Ori = 0b110,

        /// And immediate
        Andi = 0b111,
    };
    Invalid
}

emu_enum! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32OpImmFunct7;
    u32;
    {
        /// Shift right logical immediate
        Srli = 0b0000000,

        /// Shift right arithmetic immediate
        Srai = 0b0100000,

        // Bitmanip instructions
        Bitmanip = 0b011_0000,

        Orc = 0b001_0100,

        Rev8 = 0b011_0100,
    };
    Invalid
}

impl RvInstr32OpImmFunct7 {
    /// Shift Left Logical function
    #[allow(non_upper_case_globals)]
    pub const Slli: RvInstr32OpImmFunct7 = RvInstr32OpImmFunct7::Srli;
}

emu_enum! {
    /// RISCV Store Opcode Functions
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32StoreFunct3;
    u32;
    {
        /// Store Byte function
        Sb = 0b000,

        /// Store Half Word function
        Sh = 0b001,

        /// Store Word function
        Sw = 0b010,
    };
    Invalid
}

emu_enum! {
    /// RISCV Store Opcode Functions
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32OpFunct3;
    u32;
    {
        /// Function Zero
        Zero = 0b000,

        /// Function One
        One = 0b001,

        /// Function Two
        Two = 0b010,

        /// Function Three
        Three = 0b011,

        /// Function Four
        Four = 0b100,

        /// Function Five
        Five= 0b101,

        /// Function Six
        Six = 0b110,

        /// Function Seven
        Seven = 0b111,
    };
    Invalid
}

emu_enum! {
    /// RISCV Branch Opcode Functions
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32BranchFunct3;
    u32;
    {
        /// Branch on equal function
        Beq = 0b000,

        /// Branch on not equal function
        Bne = 0b001,

        /// Branch on less than
        Blt = 0b100,

        /// Branch on greater than equal
        Bge = 0b101,

        /// Branch on less than unsigned
        Bltu = 0b110,

        /// Branch on greater than equal unsigned
        Bgeu = 0b111,
    };
    Invalid
}

emu_enum! {
    /// RISCV System Opcode Functions
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32SystemFunct3;
    u32;
    {
        /// Private functions
        Priv =  0b000,

        /// CSR Read Write
        Csrrw = 0b001,

        /// CSR Read and Set bits
        Csrrs = 0b010,

        /// CSR Read and Clear bits
        Csrrc = 0b011,

        /// CSR Read Write Immediate
        Csrrwi = 0b101,

        /// CSR Read and Set bits Immediate
        Csrrsi = 0b110,

        /// CSR Read and Clear bits Immediate
        Csrrci = 0b111,

    };
    Invalid
}

emu_enum! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32OpFunct10;
    u32;
    {
        // Format is YYY_YYYY_XXX, where 'YYY_YYYY' is func7 and XXX is func3
        Add     = 0b000_0000_000,
        Sll     = 0b000_0000_001,
        Slt     = 0b000_0000_010,
        Sltu    = 0b000_0000_011,
        Xor     = 0b000_0000_100,
        Srl     = 0b000_0000_101,
        Or      = 0b000_0000_110,
        And     = 0b000_0000_111,
        Mul     = 0b000_0001_000,
        Mulh    = 0b000_0001_001,
        Mulhsu  = 0b000_0001_010,
        Mulhu   = 0b000_0001_011,
        Div     = 0b000_0001_100,
        Divu    = 0b000_0001_101,
        Rem     = 0b000_0001_110,
        Remu    = 0b000_0001_111,
        Sub     = 0b010_0000_000,
        Sra     = 0b010_0000_101,
    };
    Invalid
}

emu_enum! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32OpFunct7;
    u32;
    {
        /// Add function
        Add = 0b000_0000,

        /// Multiply function
        Mul = 0b000_0001,

        /// Sub function
        Sub = 0b010_0000,

        // Min, Max, or Clmul
        MinMaxClmul = 0b000_0101,

        Zext = 0b000_0100,

        Rotate = 0b011_0000,
    };
    Invalid
}

#[allow(non_upper_case_globals)]
impl RvInstr32OpFunct7 {
    /// Shift Left Logical function
    pub const Sll: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Set Less Than
    pub const Slt: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Set Less Than Unsigned
    pub const Sltu: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Xor
    pub const Xor: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Shift Right Logical function
    pub const Srl: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Shift Right Arithmetic function
    pub const Sra: RvInstr32OpFunct7 = RvInstr32OpFunct7::Sub;

    /// Or function
    pub const Or: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// And function
    pub const And: RvInstr32OpFunct7 = RvInstr32OpFunct7::Add;

    /// Multiply High function
    pub const Mulh: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Multiply High signed and unsigned function
    pub const Mulhsu: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Multiply High unsigned function
    pub const Mulhu: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Divide function
    pub const Div: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Divide Unsigned function
    pub const Divu: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Remainder function
    pub const Rem: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    /// Remainder Unsigned function
    pub const Remu: RvInstr32OpFunct7 = RvInstr32OpFunct7::Mul;

    // AND with inverted operand
    pub const Andn: RvInstr32OpFunct7 = RvInstr32OpFunct7::Sub;

    // OR with inverted operand
    pub const Orn: RvInstr32OpFunct7 = RvInstr32OpFunct7::Sub;

    // XNOR with inverted operand
    pub const Xnor: RvInstr32OpFunct7 = RvInstr32OpFunct7::Sub;
}

emu_enum! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
    pub RvInstr32SystemImm;
    u32;
    {
        /// Environment call
        Ecall = 0b0000_0000_0000,

        /// Break
        Ebreak = 0b0000_0000_0001,

        /// Mret
        Mret = 0b0011_0000_0010,
    };
    Invalid
}

bitfield! {
    /// RISCV 32-bit instruction
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct RvInstr32(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit I-Type instruction
    pub struct RvInstr32I(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Destination Register
    pub from into XReg, rd, set_rd: 11, 7;

    /// Opcode function
    pub from into u32, funct3, set_funct3: 14, 12;

    /// Source Register
    pub from into XReg, rs, set_rs: 19, 15;

    /// Shift Amount
    pub u32, shamt, set_shamt: 24, 20;

    /// Opcode function
    pub u32, funct7, set_funct7: 31, 25;

    pub u32, funct5, set_funct5: 24, 20;

    /// Immediate value
    pub i32, imm, set_imm: 31, 20;

    /// Immediate value
    pub u32, uimm, set_uimm: 31, 20;
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit U-Type instruction
    pub struct RvInstr32U(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Destination Register
    pub from into XReg, rd, set_rd: 11, 7;

    /// Immediate value
    pub i32, imm, set_imm: 31, 12;
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit S-Type instruction
    pub struct RvInstr32S(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Immediate value
    pub u32, imm11_7, set_imm11_7: 11, 7;

    /// Opcode function
    pub from into u32, funct3, set_funct3: 14, 12;

    /// Source Register 1
    pub from into XReg, rs1, set_rs1: 19, 15;

    /// Source Register 2
    pub from into XReg, rs2, set_rs2: 24, 20;

    /// Immediate value
    pub i32, imm31_25, set_imm31_25: 31, 25;
}

impl RvInstr32S {
    pub fn imm(&self) -> i32 {
        (self.imm31_25() << 5) | (self.imm11_7() as i32)
    }

    #[allow(dead_code)]
    pub fn set_imm(&mut self, imm: i32) {
        let imm = imm as u32;
        self.set_imm11_7(imm.bit_range(4, 0));
        self.set_imm31_25(imm.bit_range(11, 5));
    }
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit R-Type instruction
    pub struct RvInstr32R(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Destination Register
    pub from into XReg, rd, set_rd: 11, 7;

    /// Opcode function
    pub from into u32, funct3, set_funct3: 14, 12;

    /// Source Register 1
    pub from into XReg, rs1, set_rs1: 19, 15;

    /// Source Register 2
    pub from into XReg, rs2, set_rs2: 24, 20;

    /// Opcode function
    pub from into u32, funct7, set_funct7: 31, 25;
}
impl RvInstr32R {
    pub fn set_funct10(&mut self, val: RvInstr32OpFunct10) {
        self.set_funct3(u32::from(val) & 0b000_0000_111);
        self.set_funct7((u32::from(val) & 0b111_1111_000) >> 3);
    }
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit B-Type instruction
    pub struct RvInstr32B(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Immediate
    pub from into u32, imm7, set_imm7: 7, 7;

    /// Immediate
    pub from into u32, imm11_8, set_imm11_8: 11, 8;

    /// Opcode function
    pub from into u32, funct3, set_funct3: 14, 12;

    /// Source Register 1
    pub from into XReg, rs1, set_rs1: 19, 15;

    /// Source Register 2
    pub from into XReg, rs2, set_rs2: 24, 20;

    /// Immediate
    pub from into u32, imm30_25, set_imm30_25: 30, 25;

    /// Immediate
    pub i32, imm31, set_imm31: 31, 31;
}

impl RvInstr32B {
    pub fn imm(&self) -> u32 {
        let mut imm = 0u32;
        imm.set_bit_range(31, 12, self.imm31());
        imm.set_bit_range(11, 11, self.imm7());
        imm.set_bit_range(10, 5, self.imm30_25());
        imm.set_bit_range(4, 1, self.imm11_8());
        imm.set_bit_range(0, 0, 0);
        imm
    }

    #[allow(dead_code)]
    pub fn set_imm(&mut self, imm: u32) {
        self.set_imm7(imm.bit_range(11, 11));
        self.set_imm11_8(imm.bit_range(4, 1));
        self.set_imm30_25(imm.bit_range(10, 5));
        self.set_imm31(imm.bit_range(12, 12));
    }
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV 32-bit J-Type instruction
    pub struct RvInstr32J(u32);

    /// Opcode
    pub from into RvInstr32Opcode, opcode, set_opcode: 6, 0;

    /// Destination Register
    pub from into XReg, rd, set_rd: 11, 7;

    /// Immediate value
    pub u32, imm19_12, set_imm19_12: 19, 12;

    /// Immediate value
    pub u32, imm20, set_imm20_20: 20, 20;

    /// Immediate value
    pub u32, imm30_21, set_imm30_21: 30, 21;

    /// Immediate
    pub i32, imm31, set_imm31: 31, 31;
}

impl RvInstr32J {
    pub fn imm(&self) -> u32 {
        let mut imm = 0u32;
        imm.set_bit_range(31, 20, self.imm31());
        imm.set_bit_range(19, 12, self.imm19_12());
        imm.set_bit_range(11, 11, self.imm20());
        imm.set_bit_range(10, 1, self.imm30_21());
        imm
    }

    #[allow(dead_code)]
    pub fn set_imm(&mut self, imm: u32) {
        self.set_imm31(imm.bit_range(20, 20));
        self.set_imm30_21(imm.bit_range(10, 1));
        self.set_imm20_20(imm.bit_range(11, 11));
        self.set_imm19_12(imm.bit_range(19, 12));
    }
}

/// RISCV Instruction
pub enum RvInstr {
    // 32-bit Instruction
    Instr32(u32),

    // 16-bit Instruction
    Instr16(u16),
}

bitfield! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    /// RISCV Machine Mode Status Register
    pub struct RvMStatus(u32);

    /// Machine Mode Interrupt Enable
    pub u32, mie, set_mie: 3, 3;

    /// Machine Mode Previous Interrupt Enable
    pub u32, mpie, set_mpie: 7, 7;
}
