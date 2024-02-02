use crate::Bus;

pub struct CpuBus<P: Bus, A: Bus, G: Bus> {
    memory: [u8; 0x0800],
    ppu: P,
    apu: A,
    gamepack: G,
}

impl<P: Bus, A: Bus, G: Bus> CpuBus<P, A, G> {
    pub const fn new(ppu: P, apu: A, gamepack: G) -> Self {
        Self {
            memory: [0u8; 0x0800],
            ppu,
            apu,
            gamepack,
        }
    }
}

impl<P: Bus, A: Bus, G: Bus> Bus for CpuBus<P, A, G> {
    fn read(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            self.memory[(address & 0x07FF) as usize]
        } else if address <= 0x3FFF {
            self.ppu.read(address & 0x0007)
        } else if address <= 0x4017 {
            self.apu.read(address & 0x0017)
        } else if address <= 0x401F {
            0
        }
        else {
            self.gamepack.read(address)
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        if address <= 0x1FFF {
            self.memory[(address & 0x07FF) as usize] = data;
        } else if address <= 0x3FFF {
            self.ppu.write(address & 0x0007, data);
        } else if address <= 0x4017 {
            self.apu.write(address & 0x0017, data);
        } else if address <= 0x401F {
        }
        else {
            self.gamepack.write(address, data);
        }
    }
}