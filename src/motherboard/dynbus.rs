use crate::Bus;

enum Device {
    SubDevice(Box<dyn Bus>),
    Mirror(u16),
}

struct Port {
    start: u16,
    end: u16,
    device: Device,
}

/// A dyncmic Bus multiplexor that take ownership of multiple bus devices
/// and exposes a multiplexed bus.
/// 
/// The bus can be configured in runtime (dynamic)
pub struct DynBus {
    ports: Vec<Port>,
}

impl DynBus {

    /// Creates a new dynamic with no connected devices
    pub fn new() -> Self {
        Self { ports: Vec::new() }
    }

    /// Mount a device to the bus.
    /// 
    /// Moves ownership of the sub_device to the dyncmic bus.
    /// 
    /// * `start` - Startig address to map
    /// * `end` - Ending address to map (inclusive)
    /// * `sub_device` - The device to be mounted
    pub fn mount_device(&mut self, start: u16, end: u16, sub_device: impl Bus + 'static) {
        self.mount(Port {
            start,
            end,
            device:Device::SubDevice(Box::new(sub_device)),
        });
    }

    pub fn mirror(&mut self, start: u16, end:u16, base: u16) {
        self.mount(Port{
            start,
            end,
            device: Device::Mirror(base),
        });
    }

    fn mount(&mut self, port: Port) {
        self.ports.push(port);
        self.validate();
    }

    fn validate(&mut self) {
        self.ports.sort_by_key(|d| d.start);
        let mut prev = 0u16;
        for dev in &self.ports {
            if (dev.start) < prev {
                panic!("Device address ranges overlap");
            }
            // BUG: This Will not validate properly on some edge cases
            prev = dev.end.wrapping_add(1);
        }
    }

    fn lookup_port(&self, address: u16) -> Option<&Port> {
        for port in &self.ports {
            if port.start <= address && address <= port.end {
                return Some(port);
            }
        }
        None
    }

    fn lookup_port_mut(&mut self, address: u16) -> Option<&mut Port> {
        for port in &mut self.ports {
            if port.start <= address && address <= port.end {
                return Some(port);
            }
        }
        None
    }
}

impl Bus for DynBus {
    fn read(&self, address: u16) -> u8 {
        let port = self.lookup_port(address);
        match port {
            Some(port) => match &port.device {
                Device::SubDevice(device) => device.read(address - port.start),
                Device::Mirror(base) => {
                    let off = address - port.start;
                    self.read(base + off)
                }
            },
            None => 0,
        }
    }

    fn write(&mut self, address: u16, data: u8) -> () {
        let port = self.lookup_port_mut(address);
        if let Some(port) = port {
            match &mut port.device {
                Device::SubDevice(device) => device.write(address - port.start, data),
                Device::Mirror(base) => {
                    let base = *base;
                    let off = address - port.start;
                    self.write(base + off, data);
                }
            }
            // dev.device.set(address - dev.start, data)
        }
    }
}