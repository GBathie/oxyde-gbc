use super::{Cpu, Src8, Reg8, Src16, Reg16, PostLoadOp};
use super::instruction::{Instruction, ControlOp, Condition, AluOp, BitOp, RotKind};

impl Cpu {

    /// Decodes the next instruction, increasing the PC as needed.
    /// 
    /// **Decoding algorithm:**  
    /// Uses decoding tricks for the related Z80 CPU:
    /// see [here](http://www.z80.info/decoding.htm).  
    /// See the opcodes table [here](https://meganesu.github.io/generate-gb-opcodes/)
    /// or [here](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html).
    /// 
    /// We use a few temporary variables:
    /// - `x` is the higher two bits of `opcode`,
    /// - `y` the next three and
    /// - `z` the last three bits.
    /// 
    /// The case `x = cst` corresponds to a block of 4 consecutive lines in the opcodes table.
    /// Then, `z = cst` corresponds to two columns of 4 instructions, at index `cst` and `cst + 8`.
    /// The lower bit of `y` (hereafter denoted `q`, and the upper two bits denoted `p`) allows
    /// distinguishing between columns `cst` and `cst+8`.
    /// This is useful for Inc/Dec (`0x03` vs `0x0B`).
    /// To handle the case where the split is upper part of the two columns vs lower part,
    /// we use `s` and `r`, respectively the upper bit and the lower two bits of `y`,
    /// e.g. for opcode `0x20, 0x30, 0x28` and `0x38`. 
    pub(super) fn fetch_decode(&mut self) -> Instruction {
        let mut opcode = self.fetch();

        if opcode == 0xCB {
            opcode = self.fetch();
            return decode_bit_op(opcode);
        }
        let (x, y, z) = decompose(opcode);
        match x {
            0x0 => decode_mixed(self, y, z),
            0x1 => decode_load(y, z),
            0x2 => decode_alu(y, z),
            0x3 => decode_control(self, y, z),
            _ => unreachable!(),
        }
    }
}


fn decompose(opcode: u8) -> (usize, usize, usize) {
    let x = ((opcode >> 6) & 0b11) as usize;
    let y = ((opcode >> 3) & 0b111) as usize;
    let z = (opcode        & 0b111) as usize;

    (x, y, z)
}


fn read_imm_u8(cpu: &mut Cpu) -> u8 {
    cpu.fetch()
}

fn read_imm_i8(cpu: &mut Cpu) -> i8 {
    cpu.fetch() as i8
}

fn read_imm_u16(cpu: &mut Cpu) -> u16 {
    let mut val: u16 = cpu.fetch() as u16;
    val |= (cpu.fetch() as u16) << 8;
    val
}

const R : [Src8; 8] = 
    [Src8::Register(Reg8::B), Src8::Register(Reg8::C), Src8::Register(Reg8::D), Src8::Register(Reg8::E),
     Src8::Register(Reg8::H), Src8::Register(Reg8::L), Src8::Reg16Addr(Reg16::HL), Src8::Register(Reg8::A)];

fn decode_load(y: usize, z: usize) -> Instruction {
    if y == 6 && z == 6 {
        Instruction::Halt
    } else {
        Instruction::Load8 { target: R[y], source: R[z] }
    } 
}

const RP  : [Src16; 4] =
    [Src16::Register(Reg16::BC), Src16::Register(Reg16::DE),
     Src16::Register(Reg16::HL), Src16::Register(Reg16::SP)];
const RP2 : [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF];

const RP3 : [Src8; 4] = [Src8::Reg16Addr(Reg16::BC), Src8::Reg16Addr(Reg16::DE), 
                         Src8::Reg16AddrOp(Reg16::HL, PostLoadOp::Inc), 
                         Src8::Reg16AddrOp(Reg16::HL, PostLoadOp::Dec)];

fn decode_mixed(cpu: &mut Cpu, y: usize, z: usize) -> Instruction {
    let p = y >> 1;
    let q = y & 0b1;

    let r = y >> 2; 
    let s = y & 0b11;

    match z {
        0x0 if r == 0 => match s {
            0x0 => Instruction::Nop,
            0x1 => Instruction::Load16 { target: Src16::ConstAddr(read_imm_u16(cpu)), source: Reg16::SP.into() },
            0x2 => {
                // The `Stop` instruction is actually two bytes long
                let _ = cpu.fetch();
                Instruction::Stop
            },
            0x3 => Instruction::Control(ControlOp::JumpRel(read_imm_i8(cpu)), Condition::None),
            _ => unreachable!(),
        },
        0x0 if r == 1 => Instruction::Control(ControlOp::JumpRel(read_imm_i8(cpu)), COND[s]),
        0x1 if q == 1 => Instruction::Load16 {
            target: RP[p], 
            source: Src16::Const(read_imm_u16(cpu)) 
        },
        0x1 => Instruction::Alu16(AluOp::Add, RP[p]),
        0x2 if q == 0 => Instruction::Load8 { target: RP3[p], source: Src8::Register(Reg8::A) },
        0x2 if q == 1 => Instruction::Load8 { target: Src8::Register(Reg8::A), source: RP3[p] },
        0x3 if q == 0 => Instruction::Alu16(AluOp::Inc, RP[p]),
        0x3 if q == 1 => Instruction::Alu16(AluOp::Dec, RP[p]),
        0x4 => Instruction::Alu8(AluOp::Inc, R[y]),
        0x5 => Instruction::Alu8(AluOp::Dec, R[y]),
        0x6 => Instruction::Load8 { target: R[y], source: Src8::Const(read_imm_u8(cpu)) },
        0x7 if r == 0 => Instruction::RotA(BitOp::Rot(ROT[s])),
        0x7 if r == 1 => match s {
            0x0 => Instruction::DAA,
            0x1 => Instruction::ComplA,
            0x2 => Instruction::CarryFlag { set: true },
            0x3 => Instruction::CarryFlag { set: false },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

const ALU : [AluOp; 8] = 
    [AluOp::Add, AluOp::AddC, AluOp::Sub, AluOp::SbC,
     AluOp::And, AluOp::Xor, AluOp::Or, AluOp::Cmp];

fn decode_alu(y: usize, z: usize) -> Instruction {
    Instruction::Alu8(ALU[y], R[z])
}

const COND : [Condition; 4] = [Condition::NZ, Condition::Z, Condition::NC, Condition::C];

fn decode_control(cpu: &mut Cpu, y: usize, z: usize) -> Instruction {
    let p = y >> 1;
    let q = y & 0b1;

    let r = y >> 2; 
    let s = y & 0b11;

    match z {
        0x0 if r == 0 => Instruction::Control(ControlOp::Ret, COND[s]),
        0x0 if r == 1 => match s {
            0x0 => Instruction::Load8 { target: Src8::FFOffsetRegC, source: Reg8::A.into() },
            0x1 => Instruction::Load8 { target: Src8::ConstAddr(read_imm_u16(cpu)), source: Reg8::A.into() },
            0x2 => Instruction::Load8 { target: Reg8::A.into(), source: Src8::FFOffsetRegC },
            0x3 => Instruction::Load8 { target: Reg8::A.into(), source: Src8::ConstAddr(read_imm_u16(cpu)) },
            _ => unreachable!(),
        },
        0x1 if q == 0 => Instruction::Pop(RP2[p]),
        0x1 if q == 1 => match p {
            0x0 => Instruction::Control(ControlOp::Ret, Condition::None),
            0x1 => Instruction::Control(ControlOp::RetI, Condition::None),
            0x2 => Instruction::Control(ControlOp::JumpHL, Condition::None),
            0x3 => Instruction::Load16 { target: Reg16::SP.into(), source: Reg16::HL.into() },
            _ => unreachable!(),
        },
        0x2 if r == 0 => Instruction::Control(ControlOp::Jump(read_imm_u16(cpu)), COND[s]),
        0x2 if r == 1 => match s {
            0x0 => Instruction::Load8 { target: Src8::FFOffsetAddr(read_imm_u8(cpu)), source: Reg8::A.into() },
            0x1 => Instruction::Load16 { target: Reg16::SP.into(), source: Src16::SpOffset(read_imm_i8(cpu)) },
            0x2 => Instruction::Load8 { target: Reg8::A.into(), source: Src8::FFOffsetAddr(read_imm_u8(cpu)) },
            0x3 => Instruction::Load16 { target: Reg16::HL.into(), source: Src16::SpOffset(read_imm_i8(cpu)) },
            _ => unreachable!(),
        },
        0x3 => match y {
            0x0 => Instruction::Control(ControlOp::Jump(read_imm_u16(cpu)), Condition::None),
            0x6 => Instruction::InterruptEnable { enable: false },
            0x7 => Instruction::InterruptEnable { enable: true },
            _ => unreachable!(),
        },
        0x4 if r == 0 => Instruction::Control(ControlOp::Call(read_imm_u16(cpu)), COND[s]),
        0x5 if q == 0 => Instruction::Push(RP2[p]),
        0x5 if q == 1 && p == 0 => Instruction::Control(ControlOp::Call(read_imm_u16(cpu)), Condition::None),
        0x6 => Instruction::Alu8(ALU[y], Src8::Const(read_imm_u8(cpu))),
        0x7 => Instruction::RST(y),
        _ => unreachable!(),
    }
}

const ROT: [RotKind; 8] =
    [RotKind::RotLeftCarry,   RotKind::RotRightCarry,   RotKind::RotLeft,        RotKind::RotRight,
     RotKind::ShiftLeftArith, RotKind::ShiftRightArith, RotKind::ShiftLeftLogic, RotKind::ShiftRightLogic];

fn decode_bit_op(opcode: u8) -> Instruction {
    let (x, y, z) = decompose(opcode);
    Instruction::Bit(
    match x {
        0x0 => BitOp::Rot(ROT[y]),
        0x1 => BitOp::GetBit(y),
        0x2 => BitOp::ResBit(y),
        0x3 => BitOp::SetBit(y),
        _ => unreachable!(),
    }, R[z])
}