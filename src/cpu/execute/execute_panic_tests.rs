use super::Cpu;

#[test]
fn test_opcode_00_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x00);

    cpu.step();
}

#[test]
fn test_opcode_01_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x01);

    cpu.step();
}

#[test]
fn test_opcode_02_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x02);

    cpu.step();
}

#[test]
fn test_opcode_03_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x03);

    cpu.step();
}

#[test]
fn test_opcode_04_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x04);

    cpu.step();
}

#[test]
fn test_opcode_05_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x05);

    cpu.step();
}

#[test]
fn test_opcode_06_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x06);

    cpu.step();
}

#[test]
fn test_opcode_07_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x07);

    cpu.step();
}

#[test]
fn test_opcode_08_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x08);

    cpu.step();
}

#[test]
fn test_opcode_09_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x09);

    cpu.step();
}

#[test]
fn test_opcode_0a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0A);

    cpu.step();
}

#[test]
fn test_opcode_0b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0B);

    cpu.step();
}

#[test]
fn test_opcode_0c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0C);

    cpu.step();
}

#[test]
fn test_opcode_0d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0D);

    cpu.step();
}

#[test]
fn test_opcode_0e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0E);

    cpu.step();
}

#[test]
fn test_opcode_0f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x0F);

    cpu.step();
}

#[test]
fn test_opcode_10_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x10);

    cpu.step();
}

#[test]
fn test_opcode_11_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x11);

    cpu.step();
}

#[test]
fn test_opcode_12_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x12);

    cpu.step();
}

#[test]
fn test_opcode_13_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x13);

    cpu.step();
}

#[test]
fn test_opcode_14_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x14);

    cpu.step();
}

#[test]
fn test_opcode_15_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x15);

    cpu.step();
}

#[test]
fn test_opcode_16_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x16);

    cpu.step();
}

#[test]
fn test_opcode_17_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x17);

    cpu.step();
}

#[test]
fn test_opcode_18_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x18);

    cpu.step();
}

#[test]
fn test_opcode_19_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x19);

    cpu.step();
}

#[test]
fn test_opcode_1a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1A);

    cpu.step();
}

#[test]
fn test_opcode_1b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1B);

    cpu.step();
}

#[test]
fn test_opcode_1c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1C);

    cpu.step();
}

#[test]
fn test_opcode_1d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1D);

    cpu.step();
}

#[test]
fn test_opcode_1e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1E);

    cpu.step();
}

#[test]
fn test_opcode_1f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x1F);

    cpu.step();
}

#[test]
fn test_opcode_20_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x20);

    cpu.step();
}

#[test]
fn test_opcode_21_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x21);

    cpu.step();
}

#[test]
fn test_opcode_22_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x22);

    cpu.step();
}

#[test]
fn test_opcode_23_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x23);

    cpu.step();
}

#[test]
fn test_opcode_24_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x24);

    cpu.step();
}

#[test]
fn test_opcode_25_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x25);

    cpu.step();
}

#[test]
fn test_opcode_26_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x26);

    cpu.step();
}

#[test]
fn test_opcode_27_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x27);

    cpu.step();
}

#[test]
fn test_opcode_28_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x28);

    cpu.step();
}

#[test]
fn test_opcode_29_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x29);

    cpu.step();
}

#[test]
fn test_opcode_2a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2A);

    cpu.step();
}

#[test]
fn test_opcode_2b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2B);

    cpu.step();
}

#[test]
fn test_opcode_2c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2C);

    cpu.step();
}

#[test]
fn test_opcode_2d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2D);

    cpu.step();
}

#[test]
fn test_opcode_2e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2E);

    cpu.step();
}

#[test]
fn test_opcode_2f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x2F);

    cpu.step();
}

#[test]
fn test_opcode_30_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x30);

    cpu.step();
}

#[test]
fn test_opcode_31_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x31);

    cpu.step();
}

#[test]
fn test_opcode_32_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x32);

    cpu.step();
}

#[test]
fn test_opcode_33_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x33);

    cpu.step();
}

#[test]
fn test_opcode_34_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x34);

    cpu.step();
}

#[test]
fn test_opcode_35_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x35);

    cpu.step();
}

#[test]
fn test_opcode_36_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x36);

    cpu.step();
}

#[test]
fn test_opcode_37_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x37);

    cpu.step();
}

#[test]
fn test_opcode_38_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x38);

    cpu.step();
}

#[test]
fn test_opcode_39_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x39);

    cpu.step();
}

#[test]
fn test_opcode_3a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3A);

    cpu.step();
}

#[test]
fn test_opcode_3b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3B);

    cpu.step();
}

#[test]
fn test_opcode_3c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3C);

    cpu.step();
}

#[test]
fn test_opcode_3d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3D);

    cpu.step();
}

#[test]
fn test_opcode_3e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3E);

    cpu.step();
}

#[test]
fn test_opcode_3f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x3F);

    cpu.step();
}

#[test]
fn test_opcode_40_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x40);

    cpu.step();
}

#[test]
fn test_opcode_41_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x41);

    cpu.step();
}

#[test]
fn test_opcode_42_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x42);

    cpu.step();
}

#[test]
fn test_opcode_43_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x43);

    cpu.step();
}

#[test]
fn test_opcode_44_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x44);

    cpu.step();
}

#[test]
fn test_opcode_45_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x45);

    cpu.step();
}

#[test]
fn test_opcode_46_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x46);

    cpu.step();
}

#[test]
fn test_opcode_47_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x47);

    cpu.step();
}

#[test]
fn test_opcode_48_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x48);

    cpu.step();
}

#[test]
fn test_opcode_49_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x49);

    cpu.step();
}

#[test]
fn test_opcode_4a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4A);

    cpu.step();
}

#[test]
fn test_opcode_4b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4B);

    cpu.step();
}

#[test]
fn test_opcode_4c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4C);

    cpu.step();
}

#[test]
fn test_opcode_4d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4D);

    cpu.step();
}

#[test]
fn test_opcode_4e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4E);

    cpu.step();
}

#[test]
fn test_opcode_4f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x4F);

    cpu.step();
}

#[test]
fn test_opcode_50_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x50);

    cpu.step();
}

#[test]
fn test_opcode_51_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x51);

    cpu.step();
}

#[test]
fn test_opcode_52_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x52);

    cpu.step();
}

#[test]
fn test_opcode_53_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x53);

    cpu.step();
}

#[test]
fn test_opcode_54_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x54);

    cpu.step();
}

#[test]
fn test_opcode_55_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x55);

    cpu.step();
}

#[test]
fn test_opcode_56_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x56);

    cpu.step();
}

#[test]
fn test_opcode_57_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x57);

    cpu.step();
}

#[test]
fn test_opcode_58_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x58);

    cpu.step();
}

#[test]
fn test_opcode_59_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x59);

    cpu.step();
}

#[test]
fn test_opcode_5a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5A);

    cpu.step();
}

#[test]
fn test_opcode_5b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5B);

    cpu.step();
}

#[test]
fn test_opcode_5c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5C);

    cpu.step();
}

#[test]
fn test_opcode_5d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5D);

    cpu.step();
}

#[test]
fn test_opcode_5e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5E);

    cpu.step();
}

#[test]
fn test_opcode_5f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x5F);

    cpu.step();
}

#[test]
fn test_opcode_60_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x60);

    cpu.step();
}

#[test]
fn test_opcode_61_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x61);

    cpu.step();
}

#[test]
fn test_opcode_62_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x62);

    cpu.step();
}

#[test]
fn test_opcode_63_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x63);

    cpu.step();
}

#[test]
fn test_opcode_64_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x64);

    cpu.step();
}

#[test]
fn test_opcode_65_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x65);

    cpu.step();
}

#[test]
fn test_opcode_66_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x66);

    cpu.step();
}

#[test]
fn test_opcode_67_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x67);

    cpu.step();
}

#[test]
fn test_opcode_68_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x68);

    cpu.step();
}

#[test]
fn test_opcode_69_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x69);

    cpu.step();
}

#[test]
fn test_opcode_6a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6A);

    cpu.step();
}

#[test]
fn test_opcode_6b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6B);

    cpu.step();
}

#[test]
fn test_opcode_6c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6C);

    cpu.step();
}

#[test]
fn test_opcode_6d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6D);

    cpu.step();
}

#[test]
fn test_opcode_6e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6E);

    cpu.step();
}

#[test]
fn test_opcode_6f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x6F);

    cpu.step();
}

#[test]
fn test_opcode_70_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x70);

    cpu.step();
}

#[test]
fn test_opcode_71_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x71);

    cpu.step();
}

#[test]
fn test_opcode_72_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x72);

    cpu.step();
}

#[test]
fn test_opcode_73_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x73);

    cpu.step();
}

#[test]
fn test_opcode_74_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x74);

    cpu.step();
}

#[test]
fn test_opcode_75_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x75);

    cpu.step();
}

#[test]
fn test_opcode_76_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x76);

    cpu.step();
}

#[test]
fn test_opcode_77_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x77);

    cpu.step();
}

#[test]
fn test_opcode_78_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x78);

    cpu.step();
}

#[test]
fn test_opcode_79_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x79);

    cpu.step();
}

#[test]
fn test_opcode_7a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7A);

    cpu.step();
}

#[test]
fn test_opcode_7b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7B);

    cpu.step();
}

#[test]
fn test_opcode_7c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7C);

    cpu.step();
}

#[test]
fn test_opcode_7d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7D);

    cpu.step();
}

#[test]
fn test_opcode_7e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7E);

    cpu.step();
}

#[test]
fn test_opcode_7f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x7F);

    cpu.step();
}

#[test]
fn test_opcode_80_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x80);

    cpu.step();
}

#[test]
fn test_opcode_81_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x81);

    cpu.step();
}

#[test]
fn test_opcode_82_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x82);

    cpu.step();
}

#[test]
fn test_opcode_83_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x83);

    cpu.step();
}

#[test]
fn test_opcode_84_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x84);

    cpu.step();
}

#[test]
fn test_opcode_85_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x85);

    cpu.step();
}

#[test]
fn test_opcode_86_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x86);

    cpu.step();
}

#[test]
fn test_opcode_87_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x87);

    cpu.step();
}

#[test]
fn test_opcode_88_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x88);

    cpu.step();
}

#[test]
fn test_opcode_89_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x89);

    cpu.step();
}

#[test]
fn test_opcode_8a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8A);

    cpu.step();
}

#[test]
fn test_opcode_8b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8B);

    cpu.step();
}

#[test]
fn test_opcode_8c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8C);

    cpu.step();
}

#[test]
fn test_opcode_8d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8D);

    cpu.step();
}

#[test]
fn test_opcode_8e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8E);

    cpu.step();
}

#[test]
fn test_opcode_8f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x8F);

    cpu.step();
}

#[test]
fn test_opcode_90_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x90);

    cpu.step();
}

#[test]
fn test_opcode_91_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x91);

    cpu.step();
}

#[test]
fn test_opcode_92_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x92);

    cpu.step();
}

#[test]
fn test_opcode_93_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x93);

    cpu.step();
}

#[test]
fn test_opcode_94_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x94);

    cpu.step();
}

#[test]
fn test_opcode_95_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x95);

    cpu.step();
}

#[test]
fn test_opcode_96_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x96);

    cpu.step();
}

#[test]
fn test_opcode_97_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x97);

    cpu.step();
}

#[test]
fn test_opcode_98_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x98);

    cpu.step();
}

#[test]
fn test_opcode_99_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x99);

    cpu.step();
}

#[test]
fn test_opcode_9a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9A);

    cpu.step();
}

#[test]
fn test_opcode_9b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9B);

    cpu.step();
}

#[test]
fn test_opcode_9c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9C);

    cpu.step();
}

#[test]
fn test_opcode_9d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9D);

    cpu.step();
}

#[test]
fn test_opcode_9e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9E);

    cpu.step();
}

#[test]
fn test_opcode_9f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0x9F);

    cpu.step();
}

#[test]
fn test_opcode_a0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA0);

    cpu.step();
}

#[test]
fn test_opcode_a1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA1);

    cpu.step();
}

#[test]
fn test_opcode_a2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA2);

    cpu.step();
}

#[test]
fn test_opcode_a3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA3);

    cpu.step();
}

#[test]
fn test_opcode_a4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA4);

    cpu.step();
}

#[test]
fn test_opcode_a5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA5);

    cpu.step();
}

#[test]
fn test_opcode_a6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA6);

    cpu.step();
}

#[test]
fn test_opcode_a7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA7);

    cpu.step();
}

#[test]
fn test_opcode_a8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA8);

    cpu.step();
}

#[test]
fn test_opcode_a9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xA9);

    cpu.step();
}

#[test]
fn test_opcode_aa_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAA);

    cpu.step();
}

#[test]
fn test_opcode_ab_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAB);

    cpu.step();
}

#[test]
fn test_opcode_ac_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAC);

    cpu.step();
}

#[test]
fn test_opcode_ad_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAD);

    cpu.step();
}

#[test]
fn test_opcode_ae_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAE);

    cpu.step();
}

#[test]
fn test_opcode_af_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xAF);

    cpu.step();
}

#[test]
fn test_opcode_b0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB0);

    cpu.step();
}

#[test]
fn test_opcode_b1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB1);

    cpu.step();
}

#[test]
fn test_opcode_b2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB2);

    cpu.step();
}

#[test]
fn test_opcode_b3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB3);

    cpu.step();
}

#[test]
fn test_opcode_b4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB4);

    cpu.step();
}

#[test]
fn test_opcode_b5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB5);

    cpu.step();
}

#[test]
fn test_opcode_b6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB6);

    cpu.step();
}

#[test]
fn test_opcode_b7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB7);

    cpu.step();
}

#[test]
fn test_opcode_b8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB8);

    cpu.step();
}

#[test]
fn test_opcode_b9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xB9);

    cpu.step();
}

#[test]
fn test_opcode_ba_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBA);

    cpu.step();
}

#[test]
fn test_opcode_bb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBB);

    cpu.step();
}

#[test]
fn test_opcode_bc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBC);

    cpu.step();
}

#[test]
fn test_opcode_bd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBD);

    cpu.step();
}

#[test]
fn test_opcode_be_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBE);

    cpu.step();
}

#[test]
fn test_opcode_bf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xBF);

    cpu.step();
}

#[test]
fn test_opcode_c0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC0);

    cpu.step();
}

#[test]
fn test_opcode_c1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC1);

    cpu.step();
}

#[test]
fn test_opcode_c2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC2);

    cpu.step();
}

#[test]
fn test_opcode_c3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC3);

    cpu.step();
}

#[test]
fn test_opcode_c4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC4);

    cpu.step();
}

#[test]
fn test_opcode_c5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC5);

    cpu.step();
}

#[test]
fn test_opcode_c6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC6);

    cpu.step();
}

#[test]
fn test_opcode_c7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC7);

    cpu.step();
}

#[test]
fn test_opcode_c8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC8);

    cpu.step();
}

#[test]
fn test_opcode_c9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xC9);

    cpu.step();
}

#[test]
fn test_opcode_ca_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCA);

    cpu.step();
}

#[test]
fn test_opcode_cc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCC);

    cpu.step();
}

#[test]
fn test_opcode_cd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCD);

    cpu.step();
}

#[test]
fn test_opcode_ce_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCE);

    cpu.step();
}

#[test]
fn test_opcode_cf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCF);

    cpu.step();
}

#[test]
fn test_opcode_d0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD0);

    cpu.step();
}

#[test]
fn test_opcode_d1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD1);

    cpu.step();
}

#[test]
fn test_opcode_d2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD2);

    cpu.step();
}

#[test]
fn test_opcode_d4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD4);

    cpu.step();
}

#[test]
fn test_opcode_d5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD5);

    cpu.step();
}

#[test]
fn test_opcode_d6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD6);

    cpu.step();
}

#[test]
fn test_opcode_d7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD7);

    cpu.step();
}

#[test]
fn test_opcode_d8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD8);

    cpu.step();
}

#[test]
fn test_opcode_d9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD9);

    cpu.step();
}

#[test]
fn test_opcode_da_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDA);

    cpu.step();
}

#[test]
fn test_opcode_dc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDC);

    cpu.step();
}

#[test]
fn test_opcode_de_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDE);

    cpu.step();
}

#[test]
fn test_opcode_df_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDF);

    cpu.step();
}

#[test]
fn test_opcode_e0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE0);

    cpu.step();
}

#[test]
fn test_opcode_e1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE1);

    cpu.step();
}

#[test]
fn test_opcode_e2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE2);

    cpu.step();
}

#[test]
fn test_opcode_e5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE5);

    cpu.step();
}

#[test]
fn test_opcode_e6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE6);

    cpu.step();
}

#[test]
fn test_opcode_e7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE7);

    cpu.step();
}

#[test]
fn test_opcode_e8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE8);

    cpu.step();
}

#[test]
fn test_opcode_e9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE9);

    cpu.step();
}

#[test]
fn test_opcode_ea_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xEA);

    cpu.step();
}

#[test]
fn test_opcode_ee_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xEE);

    cpu.step();
}

#[test]
fn test_opcode_ef_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xEF);

    cpu.step();
}

#[test]
fn test_opcode_f0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF0);

    cpu.step();
}

#[test]
fn test_opcode_f1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF1);

    cpu.step();
}

#[test]
fn test_opcode_f2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF2);

    cpu.step();
}

#[test]
fn test_opcode_f3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF3);

    cpu.step();
}

#[test]
fn test_opcode_f5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF5);

    cpu.step();
}

#[test]
fn test_opcode_f6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF6);

    cpu.step();
}

#[test]
fn test_opcode_f7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF7);

    cpu.step();
}

#[test]
fn test_opcode_f8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF8);

    cpu.step();
}

#[test]
fn test_opcode_f9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF9);

    cpu.step();
}

#[test]
fn test_opcode_fa_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xFA);

    cpu.step();
}

#[test]
fn test_opcode_fb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xFB);

    cpu.step();
}

#[test]
fn test_opcode_fe_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xFE);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_d3_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xD3);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_db_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDB);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_dd_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xDD);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_e3_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE3);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_e4_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xE4);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_eb_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xEB);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_ec_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xEC);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_ed_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xED);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_f4_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xF4);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_fc_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xFC);

    cpu.step();
}

#[test]
#[should_panic]
fn test_opcode_fd_should_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xFD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x00_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x00);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x01_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x01);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x02_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x02);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x03_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x03);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x04_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x04);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x05_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x05);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x06_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x06);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x07_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x07);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x08_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x08);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x09_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x09);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x0f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x0F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x10_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x10);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x11_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x11);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x12_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x12);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x13_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x13);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x14_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x14);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x15_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x15);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x16_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x16);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x17_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x17);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x18_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x18);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x19_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x19);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x1f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x1F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x20_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x20);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x21_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x21);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x22_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x22);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x23_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x23);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x24_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x24);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x25_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x25);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x26_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x26);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x27_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x27);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x28_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x28);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x29_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x29);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x2f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x2F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x30_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x30);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x31_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x31);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x32_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x32);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x33_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x33);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x34_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x34);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x35_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x35);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x36_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x36);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x37_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x37);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x38_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x38);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x39_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x39);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x3f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x3F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x40_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x40);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x41_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x41);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x42_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x42);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x43_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x43);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x44_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x44);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x45_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x45);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x46_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x46);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x47_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x47);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x48_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x48);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x49_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x49);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x4f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x4F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x50_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x50);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x51_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x51);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x52_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x52);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x53_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x53);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x54_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x54);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x55_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x55);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x56_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x56);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x57_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x57);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x58_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x58);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x59_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x59);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x5f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x5F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x60_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x60);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x61_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x61);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x62_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x62);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x63_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x63);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x64_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x64);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x65_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x65);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x66_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x66);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x67_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x67);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x68_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x68);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x69_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x69);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x6f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x6F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x70_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x70);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x71_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x71);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x72_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x72);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x73_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x73);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x74_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x74);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x75_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x75);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x76_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x76);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x77_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x77);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x78_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x78);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x79_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x79);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x7f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x7F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x80_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x80);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x81_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x81);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x82_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x82);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x83_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x83);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x84_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x84);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x85_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x85);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x86_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x86);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x87_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x87);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x88_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x88);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x89_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x89);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x8f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x8F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x90_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x90);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x91_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x91);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x92_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x92);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x93_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x93);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x94_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x94);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x95_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x95);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x96_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x96);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x97_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x97);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x98_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x98);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x99_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x99);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9a_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9A);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9b_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9B);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9c_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9C);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9d_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9D);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9e_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9E);

    cpu.step();
}

#[test]
fn test_opcode_cb_0x9f_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0x9F);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xa9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xA9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xaa_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xab_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xac_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xad_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xae_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAE);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xaf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xAF);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xb9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xB9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xba_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xbb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xbc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xbd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xbe_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBE);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xbf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xBF);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xc9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xC9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xca_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xcb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xcc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xcd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xce_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCE);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xcf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xCF);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xd9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xD9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xda_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xdb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xdc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xdd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xde_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDE);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xdf_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xDF);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xe9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xE9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xea_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xEA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xeb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xEB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xec_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xEC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xed_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xED);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xee_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xEE);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xef_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xEF);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf0_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF0);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf1_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF1);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf2_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF2);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf3_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF3);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf4_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF4);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf5_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF5);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf6_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF6);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf7_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF7);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf8_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF8);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xf9_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xF9);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xfa_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xFA);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xfb_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xFB);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xfc_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xFC);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xfd_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xFD);

    cpu.step();
}

#[test]
fn test_opcode_cb_0xfe_no_panic() {
    let mut cpu = Cpu::new();
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.sp -= 2;
    cpu.memory.write(cpu.pc+1, 0xFE);

    cpu.step();
}
