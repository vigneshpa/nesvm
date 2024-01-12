use crate::Bus;

use self::ram::RAM;

pub mod ram;

pub enum Device {
    SubDevice(Box<dyn Bus>),
    Mirror(u16),
}

pub struct Port {
    start: u16,
    end: u32,
    device: Device,
}

/// A dyncmic Bus multiplexor that take ownership of multiple bus devices
/// and exposes a multiplexed bus.
pub struct DynBus {
    ports: Vec<Port>,
}

impl DynBus {
    pub fn new() -> Self {
        Self { ports: Vec::new() }
    }

    pub fn mount(&mut self, port: Port) {
        self.ports.push(port);
        self.validate();
    }

    fn validate(&mut self) {
        self.ports.sort_by_key(|d| d.start);
        let mut prev = 0u32;
        for dev in &self.ports {
            if (dev.start as u32) < prev {
                panic!("Device address ranges overlap");
            }
            prev = dev.end;
        }
    }

    fn lookup_port(&self, address: u16) -> Option<&Port> {
        for port in &self.ports {
            if port.start <= address && (address as u32) < port.end {
                return Some(port);
            }
        }
        None
    }

    fn lookup_port_mut(&mut self, address: u16) -> Option<&mut Port> {
        for port in &mut self.ports {
            if port.start <= address && (address as u32) < port.end {
                return Some(port);
            }
        }
        None
    }
}

impl Bus for DynBus {
    fn get(&self, address: u16) -> u8 {
        let port = self.lookup_port(address);
        match port {
            Some(port) => match &port.device {
                Device::SubDevice(device) => device.get(address - port.start),
                Device::Mirror(base) => {
                    let off = address - port.start;
                    self.get(base + off)
                }
            },
            None => 0,
        }
    }

    fn set(&mut self, address: u16, data: u8) -> () {
        let port = self.lookup_port_mut(address);
        if let Some(port) = port {
            match &mut port.device {
                Device::SubDevice(device) => device.set(address - port.start, data),
                Device::Mirror(base) => {
                    let base = *base;
                    let off = address - port.start;
                    self.set(base + off, data);
                }
            }
            // dev.device.set(address - dev.start, data)
        }
    }
}

pub struct DynBusBuilder {
    inner: DynBus,
}

impl DynBusBuilder {
    pub fn new() -> Self {
        Self {
            inner: DynBus::new(),
        }
    }

    pub fn mount_memory(mut self, start: u16, end: u32, capacity: usize) -> Self {
        let ram = RAM::new(capacity);
        self.inner.mount(Port {
            start,
            end,
            device: Device::SubDevice(Box::new(ram)),
        });
        self
    }

    pub fn mount_initilized_memory(mut self, start: u16, init: &[u8]) -> Self {
        let ram = RAM::from_slice(init);
        self.inner.mount(Port {
            start,
            end: start as u32 + init.len() as u32,
            device: Device::SubDevice(Box::new(ram)),
        });
        self
    }

    pub fn get(self) -> DynBus {
        self.inner
    }
}
