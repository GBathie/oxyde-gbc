print("use super::Cpu;")
print()

for opcode in range(0xFF):
    if opcode not in [0xCB,
                      0xD3, 0xDB, 0xDD,
                      0xE3, 0xE4, 0xEB, 0xEC, 0xED,
                      0xF4, 0xFC, 0xFD]:
        print(f"""
#[test]
fn test_opcode_{opcode:02x}() {{
    let mut cpu = Cpu::new();
    cpu.memory[cpu.pc] = 0x{opcode:02x};

    cpu.fetch_decode();
}}
""")
    

for opcode in range(0xFF):
    print(f"""
#[test]
fn test_opcode_cb_0x{opcode:02x}() {{
    let mut cpu = Cpu::new();
    cpu.memory[cpu.pc] = 0xCB;
    cpu.memory[cpu.pc+1] = 0x{opcode:02x};

    cpu.fetch_decode();
}}
""")