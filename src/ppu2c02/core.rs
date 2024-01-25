use crate::Bus;

pub struct Core {

}

impl Core {
    pub fn new() -> Self {
        Self { }
    }
}

impl Bus for Core {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, data: u8) {
        todo!()
    }
}