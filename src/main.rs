use nesvm::Tick;

fn main() {
    let bus = nesvm::motherboard::MotherBoard::new();
    let mut cpu = nesvm::cpu6502::CPU::new(bus);

    loop {
        cpu.tick();
    }
}
