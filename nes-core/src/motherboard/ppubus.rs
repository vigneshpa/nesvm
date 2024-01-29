use crate::Bus;

pub struct PpuBus {
    oam: [u8; 512],
    memory: [u8; 0x0800],
    pallete: [u8; 0x0020],
    gamepack: Box<dyn Bus>,
}

impl PpuBus {
    pub fn new(gamepack: impl Bus + 'static) -> Self {
        Self {
            oam: [0u8; 512],
            memory: [0u8; 0x0800],
            pallete: [0u8; 0x0020],
            gamepack: Box::new(gamepack)
        }
    }
}

impl Bus for PpuBus {
    fn read(&self, address: u16) -> u8 {
        if address < 0x2000 {
            self.gamepack.read(address)
        } else {
            0
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        todo!()
    }
}