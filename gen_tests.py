print("use super::Cpu;")

for opcode in range(0xFF):
    if opcode not in [0xCB,
                      0xD3, 0xDB, 0xDD,
                      0xE3, 0xE4, 0xEB, 0xEC, 0xED,
                      0xF4, 0xFC, 0xFD]:
        print(f"""
#[test]
fn test_opcode_{opcode:02x}_no_panic() {{
    let mut cpu = Cpu::new();
    cpu.memory.write(cpu.pc, 0x{opcode:02X});

    cpu.fetch_decode();
}}""")

for opcode in [0xD3, 0xDB, 0xDD,
               0xE3, 0xE4, 0xEB, 0xEC, 0xED,
               0xF4, 0xFC, 0xFD]:
    print(f"""
#[test]
#[should_panic]
fn test_opcode_{opcode:02x}_should_panic() {{
    let mut cpu = Cpu::new();
    cpu.memory.write(cpu.pc, 0x{opcode:02X});

    cpu.fetch_decode();
}}""")


for opcode in range(0xFF):
    print(f"""
#[test]
fn test_opcode_cb_0x{opcode:02x}_no_panic() {{
    let mut cpu = Cpu::new();
    cpu.memory.write(cpu.pc, 0xCB);
    cpu.memory.write(cpu.pc+1, 0x{opcode:02X});

    cpu.fetch_decode();
}}""")