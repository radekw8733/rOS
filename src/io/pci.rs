use pci_ids::Device;

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
    pid: u16,
    // class ID
    cid: u8,
    // subclass ID
    sid: u8
}

impl PCIDeviceID {
    pub fn return_device_name(&self) -> &'static str {
        match pci_ids::Device::from_vid_pid(self.vid, self.pid) {
            Some(dev) => dev.name(),
            None => ""
        }
    }

    pub fn return_vendor_name(&self) -> &'static str {
        match pci_ids::Device::from_vid_pid(self.vid, self.pid) {
            Some(dev) => dev.vendor().name(),
            None => ""
        }
    }

    pub fn return_subclass_name(&self) -> &'static str {
        match pci_ids::Subclass::from_cid_sid(self.cid, self.sid) {
            Some(dev) => dev.name(),
            None => ""
        }
    }

    pub fn return_class_name(&self) -> &'static str {
        match pci_ids::Subclass::from_cid_sid(self.cid, self.sid) {
            Some(dev) => dev.class().name(),
            None => ""
        }
    }
}

impl core::fmt::Debug for PCIDeviceID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PCIDeviceID")
        .field("Name", &self.return_device_name())
        .field("Vendor", &self.return_vendor_name())
        // .field("Class", &self.return_class_name())
        // .field("Subclass", &self.return_subclass_name())
        // .field("Vendor ID", &format_args!("{:X}", self.vid))
        // .field("Device ID", &format_args!("{:X}", self.pid))
        .field("Class ID", &format_args!("{:X}", self.cid))
        .field("Subclass ID", &format_args!("{:X}", self.sid))
        .finish()
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
    let register0 = read_pci_config_header(dev, 0);
    let register2 = read_pci_config_header(dev, 2);
    PCIDeviceID {
        vid: register0 as u16 & 0xFFFF,
        pid: (register0 >> 16) as u16,
        cid: (register2 >> 24) as u8,
        sid: (register2 >> 16) as u8 & 0xFF
    }
}