use super::{Reg16, Src8, Src16};

#[derive(Clone, Copy, Debug)]
pub(super) enum Instruction {
    Nop,
    Stop,
    Load8 { target: Src8, source: Src8 },
    Halt,
    Alu8(AluOp, Src8),
    RotA(BitOp),
    Alu16(AluOp, Src16),
    Control(ControlOp, Condition),
    Bit(BitOp, Src8),
    Load16 {target: Src16, source: Src16},
    DAA,
    ComplA,
    CarryFlag { set: bool },
    InterruptEnable { enable: bool},
    RST(usize),
    Pop(Reg16),
    Push(Reg16),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum AluOp {
    Add,
    AddC,
    Sub,
    SbC,
    And,
    Xor,
    Or,
    Cmp,
    Inc,
    Dec,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BitOp {
    Rot(RotKind),
    GetBit(usize),
    ResBit(usize),
    SetBit(usize),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RotKind {
    RotLeftCarry,
    RotRightCarry,
    RotLeft,
    RotRight,
    ShiftLeftArith,
    ShiftRightArith,
    // ShiftLeftLogic,
    Swap,
    ShiftRightLogic,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ControlOp {
    Ret,
    RetI,
    Jump(u16),
    JumpHL,
    JumpRel(i8),
    Call(u16),
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Condition {
    NZ,
    Z,
    NC,
    C,
    None,
}