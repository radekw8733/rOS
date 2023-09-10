use crate::arch::{io_outl, io_inl};

pub struct PCIDeviceSelector {
    bus: u8,
    device: u8,
    function: u8
}

impl PCIDeviceSelector {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function
        }
    }
}

pub struct PCIDeviceID {
    // vendor ID
    vid: u16,
    // product ID
    pid: u16
}

impl PCIDeviceID {
    pub fn _new(vid: u16, pid: u16) -> Self { Self { vid, pid } }
}

impl core::fmt::Debug for PCIDeviceID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PCIDeviceID")
        .field("vendor_id", &format_args!("{:X}", self.vid))
        .field("device_id", &format_args!("{:X}", self.pid)).finish()
    }
}

pub fn is_pci_device_present(dev: &PCIDeviceSelector) -> bool {
    let response = read_pci_config_header(dev, 0);
    response != u32::MAX
}

pub fn read_pci_config_header(dev: &PCIDeviceSelector, reg_offset: u8) -> u32 {
    io_outl(0xCF8,
        1 << 31 |
        ((dev.bus as u32) << 16) |
        ((dev.device as u32) << 11) |
        ((dev.function as u32) << 8) |
        reg_offset as u32);
    io_inl(0xCFC)
}

pub fn read_pci_device_id(dev: &PCIDeviceSelector) -> PCIDeviceID {
    let response = read_pci_config_header(dev, 0);
    PCIDeviceID {
        vid: response as u16 & 0xFFFF,
        pid: (response >> 16) as u16
    }
}