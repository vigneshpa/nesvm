use crate::{gamepack::{NametableMirroring, GamePack, GamePackPPU}, Bus};

pub struct PpuBus {
    memory: Box<[u8; 0x0800]>,
    pallete: Box<[u8; 0x0020]>,
    gamepack: GamePackPPU,
    nametable_mirroring: NametableMirroring,
}

impl PpuBus {
    pub fn new(gamepack: GamePack) -> Self {
        Self {
            memory: Box::new([0u8; 0x0800]),
            pallete: Box::new([0u8; 0x0020]),
            gamepack: gamepack.get_ppu_half(),
            nametable_mirroring: NametableMirroring::Horizontal
        }
    }
    // Decode the nametable address (without base address) to memory index
    fn decode_nametable_address(&self, address: u16) -> usize {
        (match self.nametable_mirroring {
            NametableMirroring::Vertical => address & 0x07FF,
            NametableMirroring::Horizontal => if address < 0x0800 {
                address & 0x03FF
            } else {
                0x800 + ((address - 0x0800) & 0x03FF)
            },
        }) as usize
    }
}

impl Bus for PpuBus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..0x2000 => self.gamepack.read(address),
            0x2000..0x3000 => {
                let address = self.decode_nametable_address(address-0x2000);
                self.memory[address]
            },
            0x3000..0x3F00 => self.read(address - 0x3000 + 0x2000),
            0x3F00..0x3FFF => self.pallete[((address - 0x3F00) & 0x001F) as usize],
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..0x2000 => self.gamepack.write(address, data),
            0x2000..0x3000 => {
                let address = self.decode_nametable_address(address-0x2000);
                self.memory[address] = data;
            },
            0x3000..0x3F00 => self.write(address - 0x3000 + 0x2000, data),
            0x3F00..0x3F20 => self.pallete[((address - 0x3F00) & 0x001F) as usize] = data,
            _ => {}
        }
    }
}