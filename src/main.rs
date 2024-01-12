use nesvm::{Tick, cpu6502::CPU};

fn main() {
    let bus = nesvm::motherboard::dynbus::DynBusBuilder::new()
        .mount_memory(0, (u16::MAX as u32) + 1, (u16::MAX as usize) + 1)
        // .mount_memory(0, (u16::MAX as u32) + 1, (u16::MAX as usize) + 1) // Will panic due to overlap
        .get();

    let mut cpu = CPU::new(bus, 0);

    loop {
        cpu.tick();
    }
}
