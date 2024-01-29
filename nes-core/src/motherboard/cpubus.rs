use crate::Bus;

pub struct CpuBus {
    memory: [u8; 0x0800],
    ppu: Box<dyn Bus>,
    apu: Box<dyn Bus>,
    gamepack: Box<dyn Bus>
}

impl CpuBus {
    pub fn new(ppu: impl Bus + 'static, apu: impl Bus + 'static, gamepack: impl Bus + 'static) -> Self {
        let ppu = Box::new(ppu);
        let apu = Box::new(apu);
        let gamepack = Box::new(gamepack);
        Self {
            memory: [0u8; 0x0800],
            ppu,
            apu,
            gamepack,
        }
    }
}

impl Bus for CpuBus {
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
            self.gamepack.read(address - 0x4020)
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
            self.gamepack.write(address - 0x4020, data);
        }
    }
}