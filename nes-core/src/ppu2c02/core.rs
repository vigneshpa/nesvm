use crate::Bus;

pub struct Core {

}

impl Core {
    pub fn new() -> Self {
        Self { }
    }
}

impl Bus for Core {
    fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }
}