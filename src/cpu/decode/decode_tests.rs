use crate::cpu::{instruction::{Instruction, AluOp, ControlOp, BitOp, RotKind}, Src16, Src8};

use super::Cpu;

macro_rules! test_range {
    ($range:expr, $variant:pat $(if $cond:expr)? $(, $prefix:expr)?) => {
        let mut cpu = Cpu::new();
        $(
            cpu.memory.write(cpu.pc, $prefix);
            cpu.pc += 1;
        )?
        for i in $range {
            cpu.memory.write(cpu.pc, i);
            $(
                cpu.pc -= 1;
                let _ = $prefix;
            )?
            let instr = cpu.fetch_decode();
            println!("0x{i:x} -> {:?}", instr);
            assert!(matches!(instr, $variant $(if $cond)?));
            cpu.pc -= 1;
            
        }
    };
}

#[test]
fn test_load8() {
    test_range!((0x02..=0x3A).step_by(8), Instruction::Load8 {..});
    test_range!((0x06..=0x3E).step_by(8), Instruction::Load8 {..});
    test_range!(0x40..=0x75, Instruction::Load8 {..});
    test_range!(0x77..=0x7F, Instruction::Load8 {..});
    test_range!((0xE0..=0xF0).step_by(16), Instruction::Load8 {..});
    test_range!((0xE2..=0xFA).step_by(8), Instruction::Load8 {..});
}

#[test]
fn test_alu8() {
    test_range!((0x04..=0x3C).step_by(8), Instruction::Alu8(AluOp::Inc, _));
    test_range!((0x05..=0x3D).step_by(8), Instruction::Alu8(AluOp::Dec, _));
    test_range!(0x80..=0x87, Instruction::Alu8 (AluOp::Add, ..));
    test_range!(0x88..=0x8F, Instruction::Alu8 (AluOp::AddC, ..));
    test_range!(0x90..=0x97, Instruction::Alu8 (AluOp::Sub, ..));
    test_range!(0x98..=0x9F, Instruction::Alu8 (AluOp::SbC, ..));
    test_range!(0xA0..=0xA7, Instruction::Alu8 (AluOp::And, ..));
    test_range!(0xA8..=0xAF, Instruction::Alu8 (AluOp::Xor, ..));
    test_range!(0xB0..=0xB7, Instruction::Alu8 (AluOp::Or, ..));
    test_range!(0xB8..=0xBF, Instruction::Alu8 (AluOp::Cmp, ..));
    test_range!((0xC6..=0xFE).step_by(8), Instruction::Alu8 (_, Src8::Const(..)));
}

#[test]
fn test_load16() {
    test_range!((0x01..=0x31).step_by(16), Instruction::Load16 {source, ..} if matches!(source, Src16::Const(..)));
    test_range!(0x08..=0x08, Instruction::Load16 {..});
}

#[test]
fn test_alu16() {
    test_range!((0x03..=0x33).step_by(16), Instruction::Alu16 (AluOp::Inc, ..));
    test_range!((0x09..=0x39).step_by(16), Instruction::Alu16 (AluOp::Add, ..));
    test_range!((0x0B..=0x3B).step_by(16), Instruction::Alu16 (AluOp::Dec, ..));
}

#[test]
fn test_control() {
    test_range!((0x18..=0x38).step_by(8), Instruction::Control(ControlOp::JumpRel(..), ..));
    test_range!((0xC0..=0xD8).step_by(8), Instruction::Control(ControlOp::Ret, ..));
    test_range!((0xC2..=0xDA).step_by(8), Instruction::Control(ControlOp::Jump(..), ..));
    test_range!((0xC4..=0xDC).step_by(8), Instruction::Control(ControlOp::Call(..), ..));
    
    test_range!((0xC7..=0xFF).step_by(8), Instruction::Control(ControlOp::RST(..), ..));
}

#[test]
fn test_push_pop() {
    test_range!((0xC1..=0xF1).step_by(16), Instruction::Pop(..));
    test_range!((0xC5..=0xF5).step_by(16), Instruction::Push(..));
}

#[test]
fn test_cb_prefixed() {
    test_range!(0x00..=0x07, Instruction::Bit (BitOp::Rot (RotKind::RotLeftCarry), ..) , 0xCB);
    test_range!(0x08..=0x0F, Instruction::Bit (BitOp::Rot (RotKind::RotRightCarry), ..) , 0xCB);
    test_range!(0x10..=0x17, Instruction::Bit (BitOp::Rot (RotKind::RotLeft), ..) , 0xCB);
    test_range!(0x18..=0x1F, Instruction::Bit (BitOp::Rot (RotKind::RotRight), ..) , 0xCB);
    test_range!(0x20..=0x27, Instruction::Bit (BitOp::Rot (RotKind::ShiftLeftArith), ..) , 0xCB);
    test_range!(0x28..=0x2F, Instruction::Bit (BitOp::Rot (RotKind::ShiftRightArith), ..) , 0xCB);
    test_range!(0x30..=0x37, Instruction::Bit (BitOp::Rot (RotKind::Swap), ..) , 0xCB);
    test_range!(0x38..=0x3F, Instruction::Bit (BitOp::Rot (RotKind::ShiftRightLogic), ..) , 0xCB);
    
    test_range!(0x40..=0x7F, Instruction::Bit (BitOp::GetBit(..), ..) , 0xCB);
    test_range!(0x80..=0xBF, Instruction::Bit (BitOp::ResBit(..), ..) , 0xCB);
    test_range!(0xC0..=0xFF, Instruction::Bit (BitOp::SetBit(..), ..) , 0xCB);
}