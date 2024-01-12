use nesvm::{
    cpu6502::CPU,
    motherboard::{dynbus::DynBus, ram::RAM},
    Tick,
};

fn main() {
    let mut bus = DynBus::new();

    bus.mount_device(0x0000, 0x07FF, RAM::new(0x0800));
    bus.mirror(0x0800, 0x0FFF, 0x0000);
    bus.mirror(0x1000, 0x17FF, 0x0000);
    bus.mirror(0x1800, 0x1FFF, 0x0000);
    bus.mount_device(0x2000, 0x2007, RAM::new(0x0008)); // NES PPU registers
    for i in 0x2008..0x3FF {
        if i % 8 == 0 {
            bus.mirror(i, i + 7, 0x2000);
        }
    }
    bus.mount_device(0x4000, 0x4017, RAM::new(0x0018)); // NES APU and I/O registers
    bus.mount_device(0x4018, 0x401F, RAM::new(0x0008)); // Disabled APU and I/O
    bus.mount_device(0x4020, 0xFFFF, RAM::new(0xBFE0));

    let mut cpu = CPU::new(bus, 0);

    loop {
        cpu.tick();
    }
}
