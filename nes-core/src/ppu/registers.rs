use crate::{motherboard::ppubus::PpuBus, Bus};

use super::PPU;

pub struct PpuRegisters {
    ppu: *mut PPU<PpuBus>,
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll: (u8, u8),
    ppu_addr: (u8, u8),
    ppu_data: u8,
    oam_dma: u8,
}

impl Bus for PpuRegisters {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x00 => {
                self.ppu_ctrl
            },
            0x01 => {
                self.ppu_mask
            },
            0x02 => {
                self.ppu_status_read();
                self.ppu_status
            },
            0x03 => {
                self.oam_addr
            },
            0x04 => {
                self.oam_data_read();
                self.oam_data
            },
            0x05 => {
                self.ppu_scroll.0
            },
            0x06 => {
                self.ppu_addr.0
            },
            0x07 => {
                self.ppu_data_read();
                self.ppu_data
            },
            _ => panic!("Invalid PPU Registers connection")
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x00 => {
                self.ppu_ctrl = data;
                self.ppu_ctrl_write();
            },
            0x01 => {
                self.ppu_mask = data;
                self.ppu_mask_write();
            },
            0x02 => {
                self.ppu_status = data;
            },
            0x03 => {
                self.oam_addr = data;
                self.oam_addr_read();
            },
            0x04 => {
                self.oam_data = data;
                self.oam_data_write();
            },
            0x05 => {
                self.ppu_scroll.1 = self.ppu_scroll.0;
                self.ppu_scroll.0 = data;
                self.ppu_scroll_read();
            },
            0x06 => {
                self.ppu_addr.1 = self.ppu_addr.0;
                self.ppu_addr.0 = data;
                self.ppu_addr_write();
            },
            0x07 => {
                self.ppu_data = data;
                self.ppu_data_write();
            },
            _ => panic!("Invalid PPU Registers connection")
        }
    }
}

impl PpuRegisters {
    pub fn new(ppu: &mut PPU<PpuBus>) -> Self {
        Self {
            ppu: ppu as *mut PPU<PpuBus>,
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: (0, 0),
            ppu_addr: (0, 0),
            ppu_data: 0,
            oam_dma: 0,
        }
    }

    // Call back methods
    fn ppu_ctrl_write(&mut self) {
    }

    fn ppu_mask_write(&mut self) {
    }

    fn ppu_status_read(&self) {
    }

    fn oam_addr_read(&self) {
    }

    fn oam_data_read(&self) {
    }

    fn oam_data_write(&mut self) {
    }

    fn ppu_data_read(&self) {
    }

    fn ppu_scroll_read(&self) {
    }

    fn ppu_addr_write(&mut self) {
    }

    fn ppu_data_write(&mut self) {
    }

}