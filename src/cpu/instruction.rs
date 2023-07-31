use super::{Cpu, Reg8, Reg16, Src8, alu::AluOp, alu::{BitOp, RotKind}, Condition, ControlOp, Src16};

#[derive(Clone, Copy, Debug)]
pub(super) enum Instruction {
    Nop,
    Load8 {target: Src8, source: Src8},
    Halt,
    Alu8(AluOp, Src8),
    Alu16(AluOp, Src16),
    Control(ControlOp, Condition),
    Bit(BitOp, Src8),
    Load16 {target: Src16, source: Src16},
}

impl Cpu {

    /// Decodes the next instruction, increasing the PC as needed.
    /// 
    /// Uses decoding tricks for the related Z80 CPU:
    /// see [here](http://www.z80.info/decoding.htm).
    pub(super) fn fetch_decode(&mut self) -> Instruction {
        let mut opcode = self.fetch();

        if opcode == 0xCB {
            opcode = self.fetch();
            return decode_bit_op(opcode);
        }
        let (x, y, z) = decompose(opcode);
        match x {
            0x0 => decode_mixed(self, y, z),
            0x1 => decode_load(y,z),
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


fn decode_mixed(cpu: &mut Cpu, y: usize, z: usize) -> Instruction {
    let p = y >> 1;
    let q = y & 0b1;

    match z {
        0x0 => todo!("Noop &co"),
        0x1 => if q == 1 {
            Instruction::Load16 { target: RP[p], source: Src16::Const(read_imm_u16(cpu)) }
        } else {
            Instruction::Alu16(AluOp::Add, RP[p])
        },
        0x2 => todo!("Special Load"),
        0x3 => Instruction::Alu16(if q == 0 { AluOp::Inc } else { AluOp::Dec }, RP[p]),
        0x4 => Instruction::Alu8(AluOp::Inc, R[y]),
        0x5 => Instruction::Alu8(AluOp::Dec, R[y]),
        0x6 => Instruction::Load8 { target: R[y], source: Src8::Const(read_imm_u8(cpu)) },
        0x7 => todo!("Chelou"),
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

    
    match z {
        0x0 => Instruction::Control(ControlOp::Ret, COND[p]),
        0x1 => todo!("Pop"),
        0x2 => Instruction::Control(ControlOp::Jump(read_imm_u16(cpu)), COND[p]),
        0x4 => Instruction::Control(ControlOp::Call(read_imm_u16(cpu)), COND[p]),
        0x5 => todo!("Push"),
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