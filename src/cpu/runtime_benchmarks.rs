use test::bench::Bencher;

use super::Cpu;

#[bench]
fn test_cpu_speed(bencher: &mut Bencher) {
    let mut cpu = Cpu::new_test();
    bencher.iter(|| cpu.step());
}