use crate::Bus;

pub struct SubDevice {
    start: u16,
    end: u32,
    inner: Box<dyn Bus>,
}

impl SubDevice {
    pub fn new(start: u16, end: u32, device: impl Bus + 'static) -> Self {
        Self {
            start,
            end,
            inner: Box::new(device),
        }
    }
}

/// A de-multiplexor that take ownership of multiple bus devices
/// and exposes a multiplexed bus
pub struct MuxBus {
    devices: Vec<SubDevice>,
}

impl MuxBus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn mount(&mut self, sub_device: SubDevice) {
        self.devices.push(sub_device);
        self.validate();
    }

    fn validate(&mut self) {
        self.devices.sort_by_key(|d| d.start);
        let mut prev = 0u32;
        for dev in &self.devices {
            if (dev.start as u32) < prev {
                panic!("Device address ranges overlap");
            }
            prev = dev.end;
        }
    }

    fn lookup_device(&self, address: u16) -> Option<&SubDevice> {
        for device in &self.devices {
            if device.start <= address && (address as u32) < device.end {
                return Some(device);
            }
        }
        None
    }

    fn lookup_device_mut(&mut self, address: u16) -> Option<&mut SubDevice> {
        for device in &mut self.devices {
            if device.start <= address && (address as u32) < device.end {
                return Some(device);
            }
        }
        None
    }
}

impl Bus for MuxBus {
    fn get(&self, address: u16) -> u8 {
        let device = self.lookup_device(address);
        match device {
            Some(dev) => dev.inner.get(address - dev.start),
            None => 0,
        }
    }

    fn set(&mut self, address: u16, data: u8) -> () {
        let device = self.lookup_device_mut(address);
        if let Some(dev) = device {
            dev.inner.set(address - dev.start, data)
        }
    }
}
